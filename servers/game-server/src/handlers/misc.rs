use common::time_util;
use tracing::{debug, info, warn};
use vivian_codegen::{handlers, required_state};
use vivian_logic::system::{ClientSystemType, EOperator};
use vivian_proto::{
    BattleReportCsReq, BattleReportScRsp, EndNewbieCsReq, EndNewbieScRsp, GameLogReportCsReq,
    GameLogReportScRsp, GetMiscDataCsReq, GetMiscDataScRsp, GetNewsStandDataCsReq,
    GetNewsStandDataScRsp, ItemRewardInfo, NewsStandSignCsReq, NewsStandSignScRsp,
    PlayerOperationCsReq, PlayerOperationScRsp, ReadNewsCsReq, ReadNewsScRsp,
    ReportUiLayoutPlatformCsReq, ReportUiLayoutPlatformScRsp, SavePlayerSystemSettingCsReq,
    SavePlayerSystemSettingScRsp, SelectPostGirlCsReq, SelectPostGirlScRsp,
    SyncGlobalVariablesCsReq, SyncGlobalVariablesScRsp, VideoGetInfoCsReq, VideoGetInfoScRsp,
};

use crate::sync::SyncType;
use super::NetContext;

pub struct MiscHandler;

// Calculates the reward for a news stand sign-in based on the sign-in count.
fn get_news_stand_reward(sign_count: u32) -> Option<ItemRewardInfo> {
    let item_id = if sign_count >= 5 { 20 } else { 10 }; // Milestone: better item for 5+ sign-ins
    let base_amount = 500;
    let multiplier = 100;
    let amount = base_amount + (sign_count * multiplier); // Linear increase with sign_count

    Some(ItemRewardInfo {
        item_id,
        amount,
    })
}

#[handlers]
impl MiscHandler {
    #[required_state(BasicDataSync)]
    pub fn on_video_get_info_cs_req(
        context: &mut NetContext<'_>,
        _request: VideoGetInfoCsReq,
    ) -> VideoGetInfoScRsp {
        info!(
            component = "MiscHandler",
            step = "video_info",
            player_id = context.player.uid,
            "Processing video info request"
        );
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::BasicData)
    }

    pub fn on_save_player_system_setting_cs_req(
        _context: &mut NetContext<'_>,
        request: SavePlayerSystemSettingCsReq,
    ) -> SavePlayerSystemSettingScRsp {
        debug!(
            component = "MiscHandler",
            step = "save_settings",
            request = ?request,
            "Saving player system settings"
        );
        SavePlayerSystemSettingScRsp { retcode: 0 }
    }

    #[required_state(ExtendDataSync)]
    pub fn on_get_misc_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetMiscDataCsReq,
    ) -> GetMiscDataScRsp {
        info!(
            component = "MiscHandler",
            step = "misc_data",
            player_id = context.player.uid,
            "Processing misc data request"
        );
        context
            .player
            .sync_helper
            .remove_sync_response(SyncType::ExtendData)
    }

    pub fn on_get_news_stand_data_cs_req(
        context: &mut NetContext<'_>,
        _request: GetNewsStandDataCsReq,
    ) -> GetNewsStandDataScRsp {
        info!(
            component = "MiscHandler",
            step = "news_stand_data",
            player_id = context.player.uid,
            "Processing news stand data request"
        );
        GetNewsStandDataScRsp {
            retcode: 0,
            news_stand_data: Some(context.player.misc_model.news_stand.to_client_proto()),
        }
    }

    pub fn on_read_news_cs_req(
        context: &mut NetContext<'_>,
        _request: ReadNewsCsReq,
    ) -> ReadNewsScRsp {
        info!(
            component = "MiscHandler",
            step = "read_news",
            player_id = context.player.uid,
            "Processing read news request"
        );
        context
            .player
            .misc_model
            .news_stand
            .news_read_state
            .set(true);
        ReadNewsScRsp { retcode: 0 }
    }

    pub fn on_news_stand_sign_cs_req(
        context: &mut NetContext<'_>,
        _request: NewsStandSignCsReq,
    ) -> NewsStandSignScRsp {
        info!(
            component = "MiscHandler",
            step = "news_stand_sign",
            player_id = context.player.uid,
            "Processing news stand sign-in request"
        );

        if !context.player.misc_model.news_stand.can_sign.get() {
            warn!(
                component = "MiscHandler",
                step = "news_stand_sign",
                player_id = context.player.uid,
                "Sign-in not allowed"
            );
            return NewsStandSignScRsp {
                retcode: 1,
                ..Default::default()
            };
        }

        let sign_count = context.player.misc_model.news_stand.sign_count.get() + 1;

        context.player.misc_model.news_stand.can_sign.set(false);
        context.player.misc_model.news_stand.sign_count.set(sign_count);
        context
            .player
            .misc_model
            .news_stand
            .last_sign_time
            .set(time_util::unix_timestamp_seconds());

        // Calculate dynamic reward
        let reward = get_news_stand_reward(sign_count).unwrap_or_else(|| ItemRewardInfo {
            item_id: 10,
            amount: 8888,
        });
        info!(
            component = "MiscHandler",
            step = "news_stand_sign",
            player_id = context.player.uid,
            item_id = reward.item_id,
            amount = reward.amount,
            sign_count = sign_count,
            "Generated reward for sign-in"
        );

        // Placeholder: ItemModel::add_item not found
        warn!(
            component = "MiscHandler",
            step = "news_stand_sign",
            player_id = context.player.uid,
            item_id = reward.item_id,
            amount = reward.amount,
            "Skipping item addition due to missing ItemModel::add_item method"
        );
        // context.player.item_model.add_item(reward.item_id, reward.amount);

        NewsStandSignScRsp {
            retcode: 0,
            sign_count: sign_count as i32,
            last_sign_time: context.player.misc_model.news_stand.last_sign_time.get(),
            reward_list: vec![reward],
        }
    }

    pub fn on_select_post_girl_cs_req(
        context: &mut NetContext<'_>,
        request: SelectPostGirlCsReq,
    ) -> SelectPostGirlScRsp {
        info!(
            component = "MiscHandler",
            step = "select_post_girl",
            player_id = context.player.uid,
            "Processing post girl selection request"
        );
        let post_girl = &mut context.player.misc_model.post_girl;

        if request
            .post_girl_id_list
            .iter()
            .try_for_each(|id| post_girl.unlocked_items.contains_key(id).then_some(()))
            .is_none()
        {
            warn!(
                component = "MiscHandler",
                step = "select_post_girl",
                player_id = context.player.uid,
                "Invalid post girl IDs provided"
            );
            return SelectPostGirlScRsp { retcode: 1 };
        }

        post_girl.selected_id.clear();
        post_girl.random_toggle.set(request.post_girl_random_toggle);

        request.post_girl_id_list.into_iter().for_each(|id| {
            post_girl.selected_id.insert(id);
        });

        SelectPostGirlScRsp { retcode: 0 }
    }

    pub fn on_end_newbie_cs_req(
        context: &mut NetContext<'_>,
        request: EndNewbieCsReq,
    ) -> EndNewbieScRsp {
        info!(
            component = "MiscHandler",
            step = "end_newbie",
            player_id = context.player.uid,
            group_id = request.group_id,
            "Processing end newbie request"
        );
        context
            .player
            .misc_model
            .newbie
            .finished_groups
            .insert(request.group_id as i32);
        EndNewbieScRsp {
            retcode: 0,
            group_id: 0,
        }
    }

    pub fn on_player_operation_cs_req(
        context: &mut NetContext<'_>,
        request: PlayerOperationCsReq,
    ) -> PlayerOperationScRsp {
        info!(
            component = "MiscHandler",
            step = "player_operation",
            player_id = context.player.uid,
            system = request.system,
            operator = request.operator,
            "Processing player operation request"
        );
        let (Ok(system), Ok(operator)) = (
            ClientSystemType::try_from(request.system),
            EOperator::try_from(request.operator),
        ) else {
            warn!(
                component = "MiscHandler",
                step = "player_operation",
                player_id = context.player.uid,
                system = request.system,
                operator = request.operator,
                "Invalid player operation received"
            );
            return PlayerOperationScRsp { retcode: 1 };
        };

        debug!(
            component = "MiscHandler",
            step = "player_operation",
            player_id = context.player.uid,
            system = ?system,
            operator = ?operator,
            param = request.param,
            "Player operation processed"
        );

        if operator == EOperator::Enter {
            context
                .player
                .misc_model
                .switch
                .open_system_id
                .insert(system.into());
        }

        PlayerOperationScRsp { retcode: 0 }
    }

    pub fn on_report_ui_layout_platform(
        _context: &mut NetContext<'_>,
        _request: ReportUiLayoutPlatformCsReq,
    ) -> ReportUiLayoutPlatformScRsp {
        info!(
            component = "MiscHandler",
            step = "report_ui_layout",
            "Processing UI layout report"
        );
        ReportUiLayoutPlatformScRsp { retcode: 0 }
    }

    pub fn on_game_log_report_cs_req(
        _context: &mut NetContext<'_>,
        _request: GameLogReportCsReq,
    ) -> GameLogReportScRsp {
        info!(
            component = "MiscHandler",
            step = "game_log_report",
            "Processing game log report"
        );
        GameLogReportScRsp { retcode: 0 }
    }

    pub fn on_battle_report_cs_req(
        _context: &mut NetContext<'_>,
        _request: BattleReportCsReq,
    ) -> BattleReportScRsp {
        info!(
            component = "MiscHandler",
            step = "battle_report",
            "Processing battle report"
        );
        BattleReportScRsp { retcode: 0 }
    }

    pub fn on_sync_global_variables_cs_req(
        _context: &mut NetContext<'_>,
        _request: SyncGlobalVariablesCsReq,
    ) -> SyncGlobalVariablesScRsp {
        info!(
            component = "MiscHandler",
            step = "sync_global_vars",
            "Processing global variables sync"
        );
        SyncGlobalVariablesScRsp { retcode: 0 }
    }
}