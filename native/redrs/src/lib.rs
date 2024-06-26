use std::sync::RwLock;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use rustler::{Env, NifResult, Term};
use rustler::types::{Encoder, LocalPid};
use rustler::resource::ResourceArc;
use rustler::thread;
use rustler::wrapper::NIF_TERM;

type RedisCommand = Vec<String>;

struct State {
    client: redis::Client
}

struct Conn {
    sender: Sender<RedisExecution>
}

struct RedisExecution {
    reference: NIF_TERM,
    reply_pid: LocalPid,
    command: RedisCommand
}

mod atoms {
    rustler::atoms! {
        ok,
        error,
        redrs
    }
}

#[rustler::nif]
fn open<'a>(env: Env<'a>, url: &'a str) -> NifResult<Term<'a>> {
    match redis::Client::open(url) {
        Ok(client) => {
            let state = ResourceArc::new(State{
                client: client
            });
            Ok((atoms::ok(), state).encode(env))
        }
        Err(error) =>
            Ok((atoms::error(), format!("{}", error)).encode(env))
    }
}

#[rustler::nif(schedule = "DirtyIo")]
fn get_connection<'a>(env: Env<'a>, state: ResourceArc<State>, timeout: u64) -> NifResult<Term<'a>> {
    match state.client.get_connection_with_timeout(Duration::from_millis(timeout)) {
        Ok(conn) => {
            let (sender, receiver) = channel();
            
            spawn_handler(env, RwLock::new(conn), receiver);

            let wrap = ResourceArc::new(Conn{sender: sender});
            Ok((atoms::ok(), wrap).encode(env))
        }
        Err(error) =>
            Ok((atoms::error(), format!("{}", error)).encode(env))
    }
}

fn spawn_handler(env: Env<'_>, wconn: RwLock<redis::Connection>, receiver: Receiver<RedisExecution>) {
    thread::spawn::<thread::ThreadSpawner, _>(env, move |env: Env<'_>| {
        let mut conn = wconn.write().unwrap();

        for recv in receiver {
            let mut args = recv.command.into_iter();
            let cmd : String = args.next().unwrap();
            let mut query = redis::cmd(cmd.as_str());
            for arg in args {
                query.arg(arg);
            }

            unsafe {
                // TODO check another way to pass references
                let reference = Term::new(env, recv.reference);
                match query.query(&mut conn) {
                    Ok(result) => {
                        // TODO: how can we support more types?
                        let value : Option<String> = result;

                        let _ = env.send(&recv.reply_pid, (atoms::redrs(), atoms::ok(), reference, value).encode(env));
                    }
                    // TODO: stop on network errors
                    Err(error) => {
                        let _ = env.send(&recv.reply_pid, (atoms::redrs(), atoms::error(), reference, format!("{}", error)).encode(env));
                    }
                }
            }
        }
        
        atoms::ok().encode(env)
    });
}

#[rustler::nif(schedule = "DirtyIo")]
fn command<'a>(env: Env<'a>, conn: ResourceArc<Conn>, reference: Term, reply_pid: LocalPid, args: Term) -> NifResult<Term<'a>> {
    let args = args.decode::<rustler::ListIterator>()?.map(|earg| earg.decode::<String>().unwrap()).collect();

    match conn.sender.send(RedisExecution{reference: reference.as_c_arg(), command: args, reply_pid: reply_pid}) {
        Ok(()) => Ok(atoms::ok().encode(env)),
        Err(error) => Ok((atoms::redrs(), atoms::error(), format!("{}", error)).encode(env))
    }
}

#[rustler::nif(schedule = "DirtyIo")]    
fn close<'a>(env: Env<'a>, state: ResourceArc<State>) -> NifResult<Term<'a>> {
  drop(state);

  Ok(atoms::ok().encode(env))
}

fn load(env: Env, _: Term) -> bool {
  rustler::resource!(State, env);
  rustler::resource!(Conn, env);
  true
}

rustler::init!("Elixir.RedRSNif", [open, close, get_connection, command], load=load);
