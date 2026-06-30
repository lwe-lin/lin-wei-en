use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;
use spin_sdk::llm;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct Data{
    prompt: String
}

fn main() { }

#[http_component]
fn ai(req: Request) -> anyhow::Result<impl IntoResponse> {
    if req.method() == &Method::Options {
        return Ok(Response::builder()
            .status(204) // 204 代表 "No Content"，意思是「我知道了，一切正常」
            .header("Access-Control-Allow-Origin", "*")    // 許可來源
            .header("Access-Control-Allow-Methods", "POST") // 許可方法
            .header("Access-Control-Allow-Headers", "Content-Type") // 許可標頭
            .body(()) // 預檢請求不需要內容
            .build());
    }
    if req.method() == &Method::Post{
        let json = str::from_utf8(req.body()).unwrap_or("Hello");
        let data: Data = serde_json::from_str(json)?;

        let mut params = llm::InferencingParams::default();
        params.max_tokens = 1024;
        params.temperature = 0.3;
        // params.top_p = 0.9;
        // params.repeat_penalty = 1.1;
        // params.repeat_penalty_last_n = 64;

        let default_modele = llm::InferencingModel::Llama2Chat;
        let system = "<<SYS>>You are the Gemini 3.0 artificial intelligence model developed by Google.<</SYS>>";
        let prompt = format!("<s>[INST]{}{}[/INST]", system, data.prompt.as_str());
        let inference_res = llm::infer_with_options(default_modele, prompt.as_str(), params)?;
        Ok(Response::builder()
            .status(200)
            .header("content-type", "text/plain; charset=utf-8")
            .header("Access-Control-Allow-Origin", "*") // 這裡「絕對」也要有！
            .body(inference_res.text)
            .build())
    }else{
        Ok(Response::builder()
            .status(200)
            .header("content-type", "text/plain; charset=utf-8")
            .body("發生錯誤")
            .build())
    }
}