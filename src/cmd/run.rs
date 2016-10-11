//! The `run` command.

use args::{self, ToArgs};
use command_runner::{Command, CommandRunner};
#[cfg(test)]
use command_runner::TestCommandRunner;
use errors::*;
use ext::service::ServiceExt;
use ovr::Override;
use project::Project;

/// We implement `run` with a trait so we put it in its own module.
pub trait CommandRun {
    /// Run a specific pod as a one-shot task.
    fn run<CR>(&self,
               runner: &CR,
               ovr: &Override,
               pod: &str,
               command: Option<&args::Command>,
               opts: &args::opts::Run)
               -> Result<()>
        where CR: CommandRunner;

    /// Execute tests inside a fresh container.
    fn test<CR>(&self,
                runner: &CR,
                ovr: &Override,
                service: &str,
                command: Option<&args::Command>)
                -> Result<()>
        where CR: CommandRunner;
}

impl CommandRun for Project {
    fn run<CR>(&self,
               runner: &CR,
               ovr: &Override,
               pod: &str,
               command: Option<&args::Command>,
               opts: &args::opts::Run)
               -> Result<()>
        where CR: CommandRunner
    {
        let pod = try!(self.pod(pod)
            .ok_or_else(|| err!("Cannot find pod {}", pod)));

        // Get the single service in our pod.
        let file = try!(pod.merged_file(ovr));
        if file.services.len() != 1 {
            return Err(err!("Can only `run` pods with 1 service, {} has {}",
                            pod.name(),
                            file.services.len()));
        }
        let service = file.services.keys().next().expect("should have had a service");

        // Build and run our command.
        let command_args = if let Some(c) = command {
            c.to_args()
        } else {
            vec![]
        };
        runner.build("docker-compose")
            .args(&try!(pod.compose_args(self, ovr)))
            .arg("run")
            .args(&opts.to_args())
            .arg(service)
            .args(&command_args)
            .exec()
    }

    fn test<CR>(&self,
                runner: &CR,
                ovr: &Override,
                service_name: &str,
                command: Option<&args::Command>)
                -> Result<()>
        where CR: CommandRunner
    {
        let (pod, service_name) = try!(self.service_or_err(service_name));

        let command_args = if let Some(c) = command {
            c.to_args()
        } else {
            let service = try!(pod.service_or_err(ovr, service_name));
            try!(service.test_command()).iter().map(|s| s.into()).collect()
        };
        runner.build("docker-compose")
            .args(&try!(pod.compose_args(self, ovr)))
            .arg("run")
            .arg("--rm")
            .arg("--no-deps")
            .arg(service_name)
            .args(&command_args)
            .exec()
    }
}

#[test]
fn fails_on_a_multi_service_pod() {
    use env_logger;
    let _ = env_logger::init();
    let proj = Project::from_example("hello").unwrap();
    let ovr = proj.ovr("development").unwrap();
    let runner = TestCommandRunner::new();
    proj.output(ovr).unwrap();
    let opts = Default::default();
    assert!(proj.run(&runner, ovr, "frontend", None, &opts).is_err());
}

#[test]
fn runs_a_single_service_pod() {
    use env_logger;
    let _ = env_logger::init();
    let proj = Project::from_example("rails_hello").unwrap();
    let ovr = proj.ovr("development").unwrap();
    let runner = TestCommandRunner::new();
    proj.output(ovr).unwrap();
    let cmd = args::Command::new("rake").with_args(&["db:migrate"]);
    let mut opts = args::opts::Run::default();
    opts.allocate_tty = false;
    proj.run(&runner, ovr, "migrate", Some(&cmd), &opts).unwrap();
    assert_ran!(runner, {
        ["docker-compose",
         "-p",
         "rails_hello",
         "-f",
         proj.output_dir().join("pods/migrate.yml"),
         "run",
         "-T",
         "migrate",
         "rake",
         "db:migrate"]
    });
    proj.remove_test_output().unwrap();
}

#[test]
fn runs_tests() {
    use env_logger;
    let _ = env_logger::init();
    let proj = Project::from_example("hello").unwrap();
    let ovr = proj.ovr("test").unwrap();
    let runner = TestCommandRunner::new();
    proj.output(ovr).unwrap();

    proj.test(&runner, ovr, "frontend/proxy", None).unwrap();

    assert_ran!(runner, {
        ["docker-compose",
         "-p",
         "hellotest",
         "-f",
         proj.output_pods_dir().join("frontend.yml"),
         "run",
         "--rm",
         "--no-deps",
         "proxy",
         "echo",
         "All tests passed"]
    });

    proj.remove_test_output().unwrap();
}

#[test]
fn runs_tests_with_custom_command() {
    use env_logger;
    let _ = env_logger::init();
    let proj = Project::from_example("hello").unwrap();
    let ovr = proj.ovr("test").unwrap();
    let runner = TestCommandRunner::new();
    proj.output(ovr).unwrap();

    let cmd = args::Command::new("rspec").with_args(&["-t", "foo"]);
    proj.test(&runner, ovr, "proxy", Some(&cmd)).unwrap();

    assert_ran!(runner, {
        ["docker-compose",
         "-p",
         "hellotest",
         "-f",
         proj.output_pods_dir().join("frontend.yml"),
         "run",
         "--rm",
         "--no-deps",
         "proxy",
         "rspec",
         "-t",
         "foo"]
    });

    proj.remove_test_output().unwrap();
}
