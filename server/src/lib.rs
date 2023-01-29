use axum::Router;
use sync_wrapper::SyncWrapper;
use std::path::PathBuf;
use axum_extra::routing::SpaRouter;

#[shuttle_service::main]
async fn axum(#[shuttle_static_folder::StaticFolder] static_folder: PathBuf)
             -> shuttle_service::ShuttleAxum {

    // initialise the router using the spa router function
    // the path given resolves to "/out" because of the arg passed in
    let router = Router::new()
        .merge(SpaRouter::new("/", static_folder).index_file("index.html"));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
