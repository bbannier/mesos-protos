extern crate prost;
#[macro_use]
extern crate prost_derive;

// TODO(bbannier): Add some documentation for this module.

// We disable generation of documentation for imported files since the
// Mesos protobuf documentation does not pass rustdoc (in particular,
// some comments are mistakenly interpreted as code blocks).

#[doc(hidden)]
pub mod mesos {
    include!(concat!(env!("OUT_DIR"), "/mesos.rs"));
    pub mod agent {
        include!(concat!(env!("OUT_DIR"), "/mesos.agent.rs"));
    }
    pub mod allocator {
        include!(concat!(env!("OUT_DIR"), "/mesos.allocator.rs"));
    }
    pub mod authorization {
        include!(concat!(env!("OUT_DIR"), "/mesos.authorization.rs"));
    }
    pub mod executor {
        include!(concat!(env!("OUT_DIR"), "/mesos.executor.rs"));
    }
    pub mod fetcher {
        include!(concat!(env!("OUT_DIR"), "/mesos.fetcher.rs"));
    }
    pub mod internal {
        include!(concat!(env!("OUT_DIR"), "/mesos.internal.rs"));
        pub mod state {
            include!(concat!(env!("OUT_DIR"), "/mesos.internal.state.rs"));
        }
    }
    pub mod maintenance {
        include!(concat!(env!("OUT_DIR"), "/mesos.maintenance.rs"));
    }
    pub mod master {
        include!(concat!(env!("OUT_DIR"), "/mesos.master.rs"));
    }
    pub mod quota {
        include!(concat!(env!("OUT_DIR"), "/mesos.quota.rs"));
    }
    pub mod resource_provider {
        include!(concat!(env!("OUT_DIR"), "/mesos.resource_provider.rs"));
    }
    pub mod scheduler {
        include!(concat!(env!("OUT_DIR"), "/mesos.scheduler.rs"));
    }
    pub mod slave {
        include!(concat!(env!("OUT_DIR"), "/mesos.slave.rs"));
    }
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/mesos.v1.rs"));
        pub mod agent {
            include!(concat!(env!("OUT_DIR"), "/mesos.v1.agent.rs"));
        }
        pub mod allocator {
            include!(concat!(env!("OUT_DIR"), "/mesos.v1.allocator.rs"));
        }
        pub mod executor {
            include!(concat!(env!("OUT_DIR"), "/mesos.v1.executor.rs"));
        }
        pub mod maintenance {
            include!(concat!(env!("OUT_DIR"), "/mesos.v1.maintenance.rs"));
        }
        pub mod master {
            include!(concat!(env!("OUT_DIR"), "/mesos.v1.master.rs"));
        }
        pub mod quota {
            include!(concat!(env!("OUT_DIR"), "/mesos.v1.quota.rs"));
        }
        pub mod resource_provider {
            include!(concat!(env!("OUT_DIR"), "/mesos.v1.resource_provider.rs"));
        }
        pub mod scheduler {
            include!(concat!(env!("OUT_DIR"), "/mesos.v1.scheduler.rs"));
        }
    }
}

#[doc(hidden)]
pub mod appc {
    pub mod spec {
        include!(concat!(env!("OUT_DIR"), "/appc.spec.rs"));
    }
}

#[doc(hidden)]
pub mod docker {
    pub mod spec {
        include!(concat!(env!("OUT_DIR"), "/docker.spec.rs"));
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/docker.spec.v1.rs"));
        }
        pub mod v2 {
            include!(concat!(env!("OUT_DIR"), "/docker.spec.v2.rs"));
        }
    }
}

#[doc(hidden)]
pub mod oci {
    pub mod spec {
        pub mod image {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/oci.spec.image.v1.rs"));
            }
        }
    }
}
