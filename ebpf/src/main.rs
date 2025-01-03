#![no_std]
#![no_main]

mod context;
mod forwarder;
mod resolver;

use aya_ebpf::{
    bindings::TC_ACT_PIPE,
    macros::{classifier, map},
    maps::HashMap,
    programs::TcContext,
};
use aya_log_ebpf::error;
use common::{DnsQuery, DnsRecordA, NatKey, NatOrigin};
use context::{Context, Kind};
use forwarder::TrafficForwarder;
use resolver::DnsResolver;

#[map]
static SERVICE_REGISTRY: HashMap<DnsQuery, DnsRecordA> = HashMap::with_max_entries(1024, 0);

#[map]
static NAT_TABLE: HashMap<NatKey, NatOrigin> = HashMap::with_max_entries(1024, 0);

#[classifier]
pub fn resolver(mut ctx: TcContext) -> i32 {
    match try_resolve_dns(&mut ctx) {
        Ok(ret) => ret,
        Err(e) => {
            error!(&ctx, "error: {}", e);
            TC_ACT_PIPE
        }
    }
}

fn try_resolve_dns(ctx: &mut TcContext) -> Result<i32, &'static str> {
    let mut ctx = match Context::load(ctx) {
        Ok(ctx) => ctx,
        _ => return Ok(TC_ACT_PIPE),
    };

    match ctx.kind {
        Some(Kind::DNS) => {
            let mut dns_resolver = DnsResolver::new(&mut ctx);
            dns_resolver.handle()
        }
        _ => Ok(TC_ACT_PIPE),
    }
}

#[classifier]
pub fn ingress_forwarder(mut ctx: TcContext) -> i32 {
    match try_forward_ingress(&mut ctx) {
        Ok(ret) => ret,
        Err(e) => {
            error!(&ctx, "error: {}", e);
            TC_ACT_PIPE
        }
    }
}

fn try_forward_ingress(ctx: &mut TcContext) -> Result<i32, &'static str> {
    let mut ctx = match Context::load(ctx) {
        Ok(ctx) => ctx,
        _ => return Ok(TC_ACT_PIPE),
    };

    match ctx.kind {
        Some(Kind::TCP) => {
            let mut traffic_forwarder = TrafficForwarder::new(&mut ctx);
            traffic_forwarder.handle_ingress()
        }
        _ => Ok(TC_ACT_PIPE),
    }
}

#[classifier]
pub fn egress_forwarder(mut ctx: TcContext) -> i32 {
    match try_forward_egress(&mut ctx) {
        Ok(ret) => ret,
        Err(e) => {
            error!(&ctx, "error: {}", e);
            TC_ACT_PIPE
        }
    }
}

fn try_forward_egress(ctx: &mut TcContext) -> Result<i32, &'static str> {
    let mut ctx = match Context::load(ctx) {
        Ok(ctx) => ctx,
        _ => return Ok(TC_ACT_PIPE),
    };

    match ctx.kind {
        Some(Kind::TCP) => {
            let mut traffic_forwarder = TrafficForwarder::new(&mut ctx);
            traffic_forwarder.handle_egress()
        }
        _ => Ok(TC_ACT_PIPE),
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
