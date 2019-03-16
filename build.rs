extern crate flate2;
extern crate glob;
extern crate prost_build;
extern crate reqwest;
extern crate tar;

use flate2::read::GzDecoder;
use tar::Archive;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let include_dir = std::path::Path::new(&out_dir).join("mesos-1.7.2/include");

    if !include_dir.exists() {
        let body =
            reqwest::get("http://www.apache.org/dist/mesos/1.7.2/mesos-1.7.2.tar.gz").unwrap();

        let tar = GzDecoder::new(body);
        let mut archive = Archive::new(tar);
        archive.unpack(&out_dir).unwrap();
    }

    let inputs = [
        include_dir.join("mesos/mesos.proto"),
        include_dir.join("mesos/scheduler/scheduler.proto"),
        include_dir.join("mesos/allocator/allocator.proto"),
        include_dir.join("mesos/agent/agent.proto"),
        include_dir.join("mesos/authentication/authentication.proto"),
        include_dir.join("mesos/authorizer/acls.proto"),
        include_dir.join("mesos/authorizer/authorizer.proto"),
        include_dir.join("mesos/executor/executor.proto"),
        include_dir.join("mesos/fetcher/fetcher.proto"),
        include_dir.join("mesos/maintenance/maintenance.proto"),
        include_dir.join("mesos/master/master.proto"),
        include_dir.join("mesos/module/hook.proto"),
        include_dir.join("mesos/module/module.proto"),
        include_dir.join("mesos/quota/quota.proto"),
        include_dir.join("mesos/resource_provider/resource_provider.proto"),
        include_dir.join("mesos/slave/oversubscription.proto"),
        include_dir.join("mesos/state/state.proto"),
        include_dir.join("mesos/uri/uri.proto"),
        include_dir.join("mesos/slave/containerizer.proto"),
        include_dir.join("mesos/v1/agent/agent.proto"),
        include_dir.join("mesos/v1/allocator/allocator.proto"),
        include_dir.join("mesos/v1/executor/executor.proto"),
        include_dir.join("mesos/v1/maintenance/maintenance.proto"),
        include_dir.join("mesos/v1/master/master.proto"),
        include_dir.join("mesos/v1/mesos.proto"),
        include_dir.join("mesos/v1/quota/quota.proto"),
        include_dir.join("mesos/v1/resource_provider/resource_provider.proto"),
        include_dir.join("mesos/v1/scheduler/scheduler.proto"),
        include_dir.join("mesos/appc/spec.proto"),
        include_dir.join("mesos/docker/spec.proto"),
        include_dir.join("mesos/docker/v1.proto"),
        include_dir.join("mesos/docker/v2.proto"),
        include_dir.join("mesos/oci/spec.proto"),
    ];
    let inputs: Vec<_> = inputs.iter().map(|p| p).collect();

    // Check that above list of handled protos and the list of public protos from the tarball is
    // somewhat consistent.
    let files_in_tree = glob::glob(&include_dir.join("**/*.proto").to_string_lossy()).unwrap();
    assert_eq!(
        files_in_tree.count(),
        inputs.len(),
        "The list of handled files and files in the Mesos source tree is inconsistent"
    );

    let mut prost_build = prost_build::Config::new();

    if cfg!(feature = "serde-derive") {
        prost_build.type_attribute(".", "#[derive(Serialize, Deserialize)]");
    }

    prost_build
        .compile_protos(&inputs, &[&include_dir])
        .unwrap();
}
