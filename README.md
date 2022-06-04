# AKS Metrics

Observability for AKS with Terraform with the following logging and metrics configuration:

- Log Analytics Workspace
- OMS Agent
- Container Insights
- Monitoring Metrics Publisher

## Deploy

Start in `cd infrastructure`:

```sh
terraform init
terraform apply -auto-approve
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
https://docs.microsoft.com/en-us/azure/azure-monitor/containers/container-insights-onboard
https://docs.microsoft.com/en-us/azure/aks/monitor-aks
```