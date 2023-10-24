use nu_path::expand_tilde;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;
use serde_json::{from_str, to_value, Map};
use std::{fs, io::{self, Read, Write}, process::Command};
use clipboard::{ClipboardContext, ClipboardProvider};

pub struct Copy;

impl Copy {
    pub fn copy(&self, call: &EvaluatedCall, _input: &Value) -> Result<Value, LabeledError> {
        let path: String = call.req(0)?;
        let sn: String = call.get_flag("sn")?.unwrap_or_default();
        let column: String = call.get_flag("column")?.unwrap_or_default();
        let full_flag = {
            match column.as_str() {
                "" => {
                    true
                }
                _ => {
                    false
                }
            }
        };
        
        let path = Self::expand_tilde_and_check_file_exists(&path, call)?;

        let data = fs::read_to_string(path).expect("Unable to read file");
        let json_value: serde_json::Value = serde_json::from_str(&data).expect("Invalid JSON format");
        let mut json_datas: Vec<serde_json::Value> = Vec::new();

        match json_value {
            serde_json::Value::Array(array_val) => {
                for item in array_val.iter() {
                    if let Some(obj) = item.as_object() {
                        json_datas.push(serde_json::Value::Object(obj.clone()));
                    } else {
                        return Err(LabeledError {
                            label: "Invalid JSON format".into(),
                            msg: "Invalid JSON format".into(),
                            span: Some(call.head),
                        });
                    }
                }
            }
            serde_json::Value::Object(obj) => {
                json_datas.push(serde_json::Value::Object(obj));
            }
            _ => {
                return Err(LabeledError {
                    label: "Invalid JSON format".into(),
                    msg: "Invalid JSON format".into(),
                    span: Some(call.head),
                });
            }
        }

        // snの示すidxを取得
        let mut idx: i64 = -1;
        for json_data in json_datas.iter() {
            if let Some(obj) = json_data.as_object() {
                if let Some(sn_val) = obj.get("sn") {
                    if let Some(sn_str) = sn_val.as_str() {
                        if sn_str == sn {
                            idx = json_datas.iter().position(|x| x == json_data).unwrap() as i64;
                            break;
                        }
                    }
                }
            }
        }
        if idx == -1 {
            return Err(LabeledError {
                label: "Invalid SN".into(),
                msg: "Invalid SN".into(),
                span: Some(call.head),
            });
        }

        let idx = idx as usize;
        // full flagがある場合は、全てのデータをコピー
        // ない場合は、columnの示すデータのみコピー
        let copy_string = {
            if full_flag {
                Some(to_value(&json_datas[idx]).unwrap())
            } else {
                let json_data: serde_json::Value = json_datas[idx].clone();
                // columnで指定されているkeyのみを抽出
                let mut copy_value = None;
                if let Some(obj) = json_data.as_object() {
                    for (key, value) in obj.iter() {
                        if key == &column {
                            copy_value = Some(value.clone());
                        }
                    }
                }
                copy_value

            }
        };
        match copy_string {
            Some(copy_string) => {
                eprintln!("Copy text: {}", copy_string);
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(copy_string.to_string()).unwrap();
            }
            None => {
                return Err(LabeledError {
                    label: "Invalid column".into(),
                    msg: "Invalid column".into(),
                    span: Some(call.head),
                });
            }
        }

        Ok(Value::nothing(call.head))
    }
    
    fn expand_tilde_and_check_file_exists(
        file: &str,
        call: &EvaluatedCall,
    ) -> Result<String, LabeledError> {
        let path = expand_tilde(&file);
        if !path.exists() {
            eprintln!("File not found: {}", path.display());

            return Err(LabeledError {
                label: "File not found".into(),
                msg: "file not found".into(),
                span: Some(call.head),
            });
        }

        Ok(path.into_os_string().into_string().unwrap())
    }
}
