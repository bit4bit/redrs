use std::sync::RwLock;
use std::sync::mpsc::{channel, Receiver, Sender};

use rustler::{Env, NifResult, Term};
use rustler::types::{Encoder, LocalPid};
use rustler::resource::ResourceArc;
use rustler::thread;

type RedisCommand = Vec<String>;

struct State {
    client: redis::Client
}

struct Conn {
    sender: Sender<RedisCommand>
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
fn get_connection<'a>(env: Env<'a>, state: ResourceArc<State>, reply_pid: LocalPid) -> NifResult<Term<'a>> {
    match state.client.get_connection() {
        Ok(conn) => {
            let (sender, receiver) = channel();
            let wrap = ResourceArc::new(Conn{sender: sender});
            spawn_handler(env, RwLock::new(conn), reply_pid, receiver);
            Ok((atoms::ok(), wrap).encode(env))
        }
        Err(error) =>
            Ok((atoms::error(), format!("{}", error)).encode(env))
    }
}

fn spawn_handler(env: Env<'_>, wconn: RwLock<redis::Connection>, reply_pid: LocalPid, receiver: Receiver<RedisCommand>) {
    thread::spawn::<thread::ThreadSpawner, _>(env, move |env: Env<'_>| {
        for recv in receiver {
            println!("recv {:?}", recv);
            let mut conn = wconn.write().unwrap();
            let mut args = recv.into_iter();
            let cmd : String = args.next().unwrap();
            let mut query = redis::cmd(cmd.as_str());
            for arg in args {
                query.arg(arg);
            }
            
            match query.query(&mut conn) {
                Ok(result) => {
                    // TODO: how can we support more types?
                    let value : Option<String> = result;

                    let _ = env.send(&reply_pid, (atoms::redrs(), atoms::ok(), value.clone()).encode(env));
                    println!("send {:?}", value);
                }
                Err(error) => {
                    let _ = env.send(&reply_pid, (atoms::redrs(), atoms::error(), format!("{}", error)).encode(env));
                }
            }
        }
        
        atoms::ok().encode(env)
    });
}

#[rustler::nif(schedule = "DirtyIo")]
fn command<'a>(env: Env<'a>, conn: ResourceArc<Conn>, args: Term) -> NifResult<Term<'a>> {
    let args = args.decode::<rustler::ListIterator>()?.map(|earg| earg.decode::<String>().unwrap()).collect();

    conn.sender.send(args).unwrap();

    Ok(atoms::ok().encode(env))
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
