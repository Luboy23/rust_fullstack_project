// 导入所需的模块和处理器
use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler}; // 导入处理器
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse}; // 导入 HTTP 请求和响应模块
use std::io::prelude::*; // 导入 IO 预备函数

// 定义 Router 结构体
pub struct Router;

impl Router {
    // 路由方法，根据请求的 HTTP 方法和资源路径进行处理
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            // 如果是 GET 请求
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    // 解析 URI
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        // 如果路由以 /api 开头，则调用 Web 服务处理器
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req); // 处理请求
                            let _ = resp.send_response(stream); // 发送响应
                        }
                        // 否则，调用静态页面处理器
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req); // 处理请求
                            let _ = resp.send_response(stream); // 发送响应
                        }
                    }
                }
            },
            // 如果请求方法不是 GET，返回 404 页面
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req); // 处理请求
                let _ = resp.send_response(stream); // 发送响应
            }
        }
    }
}
