use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};
unsafe extern "C" fn ptrainer_sound_win2afushigisou(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
        } else {
            frame(agent.lua_state_agent, 50.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
            } else {
                frame(agent.lua_state_agent, 50.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                } else {
                    frame(agent.lua_state_agent, 50.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                    } else {
                        frame(agent.lua_state_agent, 50.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                        } else {
                            frame(agent.lua_state_agent, 50.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                            }
                        }
                    }
                }
            }
        }
    } else {
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
        } else {
            frame(agent.lua_state_agent, 45.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
            } else {
                frame(agent.lua_state_agent, 45.0);
                if macros::is_excute(agent) {
                    macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                } else {
                    frame(agent.lua_state_agent, 45.0);
                    if macros::is_excute(agent) {
                        macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                    } else {
                        frame(agent.lua_state_agent, 45.0);
                        if macros::is_excute(agent) {
                            macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                        } else {
                            frame(agent.lua_state_agent, 45.0);
                            if macros::is_excute(agent) {
                                macros::PLAY_SE(agent, Hash40::new("vc_ptrainer_win_pfushigisou"));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn install() {
    Agent::new("ptrainer")
        .sound_acmd("sound_win2afushigisou", ptrainer_sound_win2afushigisou)
        .install();
}
