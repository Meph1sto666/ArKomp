use crate::{
    ipc::commands::{ExecCommand, Response},
    plugin::types::operator_plugin::OperatorPlugin,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpawnOperatorCommand {
    name: String,
    plugin: Option<String>,
}

impl ExecCommand for SpawnOperatorCommand {
    fn execute(&self, ctx: &mut crate::ipc::command_context::CommandContext) -> Response {
        let tx = ctx.sender();
        let plugin = self.plugin.clone().unwrap_or(self.name.clone());
        let build_result = {
            let binding = ctx.plugin_registry().read().unwrap();
            let plugin = match crate::plugin::cast_plugin_to::<OperatorPlugin>(
                match binding.get_plugin(&plugin) {
                    Ok(p) => p,
                    Err(e) => {
                        return Response::Error(
                            json!({
                                "operator_id": self.name,
                                "plugin": plugin,
                                "reason": e.to_string(),
                            })
                            .to_string(),
                        );
                    }
                },
            ) {
                Ok(p) => p,
                Err(e) => {
                    return Response::Error(
                        json!({
                            "operator_id": self.name,
                            "plugin": plugin,
                            "reason": e.to_string(),
                        })
                        .to_string(),
                    );
                }
            };
            plugin.build(Some(self.name.clone()), tx)
        };

        match build_result {
            Ok(v) => {
                ctx.operators()
                    .write()
                    .unwrap()
                    .insert(self.name.clone(), v);
                Response::Success(
                    json!({
                        "operator_id": self.name
                    })
                    .to_string(),
                )
            }
            Err(_) => Response::Error(
                json!({
                    "operator_id": self.name,
                    "plugin": self.plugin,
                    "reason": "Operator not loaded",
                })
                .to_string(),
            ),
        }
    }
}
