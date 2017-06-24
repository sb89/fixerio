error_chain! {
    foreign_links {
        Hyper(::hyper::Error);
        Json(::serde_json::Error);
        Io(::std::io::Error);
    }
}