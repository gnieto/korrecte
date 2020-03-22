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

There are some lints that can be parametrized through a TOML file. You can copy the default `korrecte.toml` file and run the program with the `--config` flag:

```bash
cargo run -- --config /path/to/file.toml
```

## File linting

Instead of requiring a running Kubernetes cluster, `korrecte` is able to lint YAML manifests instead. Note that those lints that requires to read some state on the cluster, may not work as expected when running in this mode. For example, the `service_without_matching_labels` lints, searches all the possible matching pod, but it probably needs access to pods that are not defined on the manifest.

To lint a specific file, you can run:

```bash
cargo run -- --source file --path <path to file>
``` 

## Current lints

If the path is a directory, it will iterate over all the files and will apply the lints on each of them.
Name|Group|Description|References
---|---|---|---
environment_passwords|security|Finds passwords or api keys on object manifests.|https://kubernetes.io/docs/concepts/configuration/secret/<br>https://kubernetes.io/docs/tasks/inject-data-application/distribute-credentials-secure/
pdb_min_replicas|configuration|Checks that pod controllers associated to a pod disruption budget has at least one more replica than PDB min_unavailable. The pod controller won't be able to be rolled out, as no pod can be evicted (as min_unavailable is >= to the amount of replicas desired). This may cause that a node can not be cordoned.|https://itnext.io/kubernetes-in-production-poddisruptionbudget-1380009aaede
service_target_port|configuration|Finds services which uses numeric target ports. This lint suggests to use a named port with a string for a more semanthic configuration. This is also useful to be able to create an interface for the service and delegate to the underlying pod controller which port it exposes.|
service_without_matching_labels|configuration|Checks that services are well defined and has some matching object (defined by the service selector). A service without any matching pod is usually a symptom of a bad configuration.|
never_restart_with_liveness_probe|configuration|Finds pods which have a `Never` restart policy and have liveness probe set. Those containers which have a liveness probe will be stopped if the probe fails and it will never be restarted, which may lead the pod on a inconsistent state.|https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#container-probes
pod_requirements|security|Checks for pods without resource limits. Pods without resource limits may provoke a denial-of-service of the processes running on the same node.|
alb_ingress_controller_instance_misconfiguration|configuration|Checks that all ALB ingresses are linked to services which have compatible types with the ingress. When the ingress is configured with target-type `instance`, only `NodePort` and `LoadBalancer` types are allowed; when it's configured as `ip`, only `ClusterIP` services are allowed.|https://kubernetes-sigs.github.io/aws-alb-ingress-controller/guide/ingress/annotation/#target-type
required_labels|audit|Checks for missing required labels. Adding labels to your pods helps organizing the cluster and improves long-term maintainability.|https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#motivation
statefulset_no_grace_period|configuration|Finds stateful sets which has a pod template with graceful period equals to zero. Stateful Sets are usually used on clustered applications in which each of the components have state. This kind of application needs a proper shutdown with a given timeout, otherwise, the application may lead to an inconsistent state.|https://kubernetes.io/docs/tasks/run-application/force-delete-stateful-set-pod/#delete-pods
alb_named_sg|configuration|Finds ingresses of type ALB which uses identifiers instead of names on security group defintion. Using named security groups it's more semantic, less error prone and easy to verfiy that the configuration is correct.|https://github.com/HotelsDotCom/alb-ingress-controller/blob/37cfd6fe1f0863a6d35d83d2d2faab2c72f49e9a/docs/ingress-resources.md
overlapping_probes|configuration|Finds pods which liveness probe *may* execute before all readiness probes has been executed- Executing a liveness probe *before* the container is ready will provoke that pod change the status to failed.|https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#container-probes<br>https://github.com/kubernetes/kubernetes/issues/27114<br>https://cloud.google.com/blog/products/gcp/kubernetes-best-practices-setting-up-health-checks-with-readiness-and-liveness-probes
role_similar_names|configuration|Checks resources names which are similar to the default resources. For example, granting access to `daemon-set` instead of `daemonsets`. This usually is originated by a typo when writing role or cluster roles.|https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#motivation
hpa_no_request|configuration|Finds HPAs which are linked to some controller which contains any container that does not set the requirement fro the target metric. On those cases, the HPA emits some warnings and is not scaling the controller as required.|https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/#how-does-the-horizontal-pod-autoscaler-work

## Roadmap ideas

- Allow filtering by namespace or by name regex
- Change exit code if the `korrecte` finds any issue
- Make the application deployable, evaluate the lints continuously and create an API to retrieve them
- Add more reporting hooks. For example, statsd, datadog, prometheus, ... 