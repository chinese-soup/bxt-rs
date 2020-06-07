//! `hl`, `opfor`, `bshift`.

use std::{os::raw::*, ptr::NonNull};

use crate::{
    ffi::{playermove::playermove_s, usercmd::usercmd_s},
    hooks::engine,
    utils::{abort_on_panic, Function, MainThreadMarker},
};

pub static CMD_START: Function<unsafe extern "C" fn(*mut c_void, *mut usercmd_s, c_uint)> =
    Function::empty();
pub static PM_MOVE: Function<unsafe extern "C" fn(*mut playermove_s, c_int)> = Function::empty();

/// # Safety
///
/// This function must only be called right after `LoadEntityDLLs()` is called.
pub unsafe fn hook_entity_interface(marker: MainThreadMarker) {
    let mut functions = engine::GENTITYINTERFACE.get(marker);
    let functions = functions.as_mut();

    if let Some(pm_move) = &mut functions.pm_move {
        PM_MOVE.set(marker, Some(NonNull::new_unchecked(*pm_move as _)));
        *pm_move = PM_Move;
    }

    if let Some(cmd_start) = &mut functions.cmd_start {
        CMD_START.set(marker, Some(NonNull::new_unchecked(*cmd_start as _)));
        *cmd_start = CmdStart;
    }
}

/// # Safety
///
/// This function must only be called right before `ReleaseEntityDlls()` is called.
pub unsafe fn reset_entity_interface(marker: MainThreadMarker) {
    let mut functions = engine::GENTITYINTERFACE.get(marker);
    let functions = functions.as_mut();

    if let Some(pm_move) = &mut functions.pm_move {
        *pm_move = PM_MOVE.get(marker);
    }

    if let Some(cmd_start) = &mut functions.cmd_start {
        *cmd_start = CMD_START.get(marker);
    }
}

pub unsafe extern "C" fn CmdStart(player: *mut c_void, cmd: *mut usercmd_s, random_seed: c_uint) {
    abort_on_panic(move || {
        let marker = MainThreadMarker::new();

        CMD_START.get(marker)(player, cmd, random_seed);
    })
}

pub unsafe extern "C" fn PM_Move(ppmove: *mut playermove_s, server: c_int) {
    abort_on_panic(move || {
        let marker = MainThreadMarker::new();

        PM_MOVE.get(marker)(ppmove, server);
    })
}
