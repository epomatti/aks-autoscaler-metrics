# AKS Metrics

This an AKS setup with Terraform with the following logging and metrics configuration:

- Log Analytics Workspace
- OMS Agent
- Container Insights
- Monitoring Metrics Publisher


Metrics with AKS:

Enabling Container Insights: https://docs.microsoft.com/en-us/azure/azure-monitor/containers/container-insights-onboard

Reference: https://docs.microsoft.com/en-us/azure/aks/monitor-aks

```sh
cargo build
cargo run
```


cargo build --release

az aks get-credentials -n aks-icecream -g rg-icecream


###

```sh
# Confirm agent deployment
kubectl get ds omsagent --namespace=kube-system

# Confirm solution deployment
kubectl get deployment omsagent-rs -n=kube-system
```