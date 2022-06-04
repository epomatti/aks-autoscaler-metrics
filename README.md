# AKS Metrics

Observability for AKS with Terraform with the following logging and metrics configuration:

- Log Analytics Workspace
- OMS Agent
- Container Insights
- Monitoring Metrics Publisher
- ContainerLogV2

## Deploy

```sh
terraform init
terraform apply -chdir='infrastructure' -auto-approve
```

Once done get the credentials:

```sh
az aks get-credentials -n aks-icecream -g rg-icecream
```

Test the metrics components:

```sh
# Confirm agent deployment
kubectl get ds omsagent --namespace=kube-system

# Confirm solution deployment
kubectl get deployment omsagent-rs -n=kube-system
```

Set Container Insights to use ContainerLogV2:

```sh
kubectl apply -f container-azm-ms-agentconfig.yaml
```

Setup ContainerLogV2 to [Basic Logs](https://docs.microsoft.com/en-us/azure/azure-monitor/logs/basic-logs-configure?tabs=portal-1%2Cportal-2) to save costs.

```sh
az monitor log-analytics workspace table update --resource-group 'rg-icecream'  --workspace-name 'log-icecream' --name 'ContainerLogV2'  --plan 'Basic'
```

Deploy to Kubernetes:

```sh
kubectl apply -f kubernetes.yaml
```

Service should be running on the external address:

```sh
curl 'http://<CLUSTER_EXTERNAL_IP>:30000/api/icecream/5'
```

To load test it with K6:

```sh
docker run \
  -e "CLUSTER_EXTERNAL_IP=<EXTERNAL_IP>" \
  -e "VUS=100" \
  -e "DURATION=30s" \
  -e "K6_SLEEP=0" \
  -e "API=/api/icecream/factorial/20" \
  --rm -i grafana/k6 run - <k6.js
```


## App Development

Make sure you're in the app directory:

```sh
cd app
```

Set up the local environment:

```sh
cp config/example.env .env
```

Start the Rust server:

```sh
cargo build
cargo run
```

Test the app:

```sh
curl 'http://localhost:8080/api/icecream/5'
```


### With Docker

```sh
docker build -t icecream .
docker run -it -p 8080:8080 --rm --name icecream icecream 
```

## Reference

```
https://docs.microsoft.com/en-us/azure/aks/monitor-aks
https://docs.microsoft.com/en-us/azure/azure-monitor/containers/container-insights-onboard
```