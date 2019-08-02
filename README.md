# korrecte

Collection of lints to detect common pitfalls or security issues on your Kubernetes cluster.

It currently connects through Kubernetes API and requires that credentials are present available for the user who launches the application.


## How to run

After installing the Rust compiler and downloading the source code, run:

```bash
cargo run
```

If credentials are available, you should see a list of all the findings detected by the application. For example:

```bash
overlapping_probes on hello-node [default]. Metadata: {"liveness_start": "10s", "container": "hello-node", "readiness_max_delay": "11s"}
never_restart_with_liveness_probe on hello-node-hardcoded-env-var [test]. Metadata: {}
environment_passwords on hello-node-hardcoded-env-var [test]. Metadata: {"environment_var": "ADMIN_PASSWORD"}
environment_passwords on hello-node-hardcoded-env-var [test]. Metadata: {"environment_var": "ADMIN_TOKEN"}
environment_passwords on hello-node-hardcoded-env-var [test]. Metadata: {"environment_var": "KEY_SERVICE"}
never_restart_with_liveness_probe on hello-node-never-restart [test]. Metadata: {}
required_labels on kube-addon-manager-minikube [kube-system]. Metadata: {"missing_labels": "[\"app\"]"}
service_without_matching_labels on my-service [default]. Metadata: {}
```

## Customization

There are some lints that can be parametrized through a TOML file. At the moment, you need to place the file `korrecte.toml` on the same folder where the application is run.

## Roadmap ideas

- Allow filtering by namespace or by name regex
- Add clap support for the cli
- Change exit code if the `korrecte` finds any issue
- Make the application deployable, evaluate the lints continuously and create an API to retrieve them
- Add some reporting mechanisms. For example, statsd, datadog, prometheus, ... 