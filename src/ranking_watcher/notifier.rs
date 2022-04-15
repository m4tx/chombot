use serenity::{async_trait, client::Context, model::id::ChannelId};

use crate::ranking_watcher::usma::Ranking;

#[async_trait]
pub trait RankingUpdateNotifier<R: Send + Sync> {
    async fn notify(&self, ranking: &R);
}

pub struct ChannelMessageNotifier {
    channel_id: ChannelId,
    ctx: Context,
    message: String,
}

impl ChannelMessageNotifier {
    pub fn new(channel_id: ChannelId, ctx: Context, message: String) -> Self {
        Self {
            channel_id,
            ctx,
            message,
        }
    }

    fn format_delta(delta: i32) -> String {
        match delta {
            d if d < 0 => format!("({})", d),
            d if d > 0 => format!("(+{})", d),
            _ => String::from(""),
        }
    }

    fn build_message(&self, maybe_ranking: &Option<Ranking>) -> String {
        let mut base = self.message.clone();
        if let Some(ranking) = maybe_ranking {
            let ppl = ranking
                .iter()
                .filter(|entry| entry.pos_diff != 0 || entry.points_diff != 0)
                .map(|e| {
                    format!(
                        "• {}{} / {} / {}{} pkt",
                        e.pos,
                        Self::format_delta(e.pos_diff),
                        e.name,
                        e.points,
                        Self::format_delta(e.points_diff)
                    )
                })
                .collect::<Vec<String>>();
            base.push_str("\n\n");
            base.push_str(ppl.join("\n").as_str());
        }
        base
    }
}

#[async_trait]
impl RankingUpdateNotifier<Option<Ranking>> for ChannelMessageNotifier {
    async fn notify(&self, ranking: &Option<Ranking>) {
        self.channel_id
            .send_message(&self.ctx, |m| {
                m.content(self.build_message(ranking).as_str())
            })
            .await
            .unwrap();
    }
}
