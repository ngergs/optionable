#![cfg(all(test, feature = "k8s_openapi_convert"))]

use crate::OptionableConvert;
use crate::k8s_openapi::api::{
    apps::v1::{DeploymentAc, DeploymentSpecAc},
    core::v1::{ContainerAc, EnvVarAc, PodSpecAc, PodTemplateSpecAc},
};
use k8s_openapi027::api::{
    apps::v1::{Deployment, DeploymentSpec},
    core::v1::{Container, EnvVar, PodSpec, PodTemplateSpec},
};
use k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta;

#[test]
fn merge_deployment() {
    let mut deploy = Deployment {
        metadata: ObjectMeta {
            name: Some("hello".to_owned()),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(2),
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    name: Some("hello".to_owned()),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: "app".to_owned(),
                        env: Some(vec![
                            EnvVar {
                                name: "hello1".to_owned(),
                                value: Some("world1".to_owned()),
                                ..Default::default()
                            },
                            EnvVar {
                                name: "hello2".to_owned(),
                                value: Some("world2".to_owned()),
                                ..Default::default()
                            },
                        ]),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    };
    let deploy_ac = DeploymentAc {
        metadata: ObjectMeta {
            name: Some("hello".to_owned()),
            ..Default::default()
        },
        spec: Some(DeploymentSpecAc {
            template: Some(PodTemplateSpecAc {
                spec: Some(PodSpecAc {
                    containers: Some(vec![ContainerAc {
                        name: "app".to_owned(),
                        env: Some(vec![
                            EnvVarAc {
                                name: "hello3".to_owned(),
                                value: Some("world3".to_owned()),
                                ..Default::default()
                            },
                            EnvVarAc {
                                name: "hello2".to_owned(),
                                value: Some("world4".to_owned()),
                                ..Default::default()
                            },
                        ]),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    deploy.merge(deploy_ac).unwrap();
    assert_eq!(
        deploy,
        Deployment {
            metadata: ObjectMeta {
                name: Some("hello".to_owned()),
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                replicas: Some(2),
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        name: Some("hello".to_owned()),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        containers: vec![Container {
                            name: "app".to_owned(),
                            env: Some(vec![
                                EnvVar {
                                    name: "hello1".to_owned(),
                                    value: Some("world1".to_owned()),
                                    ..Default::default()
                                },
                                // merge patched
                                EnvVar {
                                    name: "hello2".to_owned(),
                                    value: Some("world5".to_owned()),
                                    ..Default::default()
                                },
                                // appended
                                EnvVar {
                                    name: "hello3".to_owned(),
                                    value: Some("world3".to_owned()),
                                    ..Default::default()
                                },
                            ]),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        }
    );
}
