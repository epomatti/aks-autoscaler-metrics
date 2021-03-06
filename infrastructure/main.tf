terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "3.8.0"
    }
  }
  backend "local" {
    path = "./.workspace/terraform.tfstate"
  }
}

provider "azurerm" {
  features {
    resource_group {
      prevent_deletion_if_contains_resources = false
    }
  }
}

### Variables ###

variable "location" {
  description = "Set .auto.tfvars to a location near you."
  type        = string
  default     = "brazilsouth"
  sensitive   = false
}

variable "aks_version" {
  description = "Kubernetes version"
  type        = string
  default     = "1.23.5"
  sensitive   = false
}

variable "aks_auto_upgrade_channel" {
  type      = string
  default   = "rapid"
  sensitive = false
}

variable "aks_vm_size" {
  description = "Kubernetes VM size"
  type        = string
  default     = "Standard_B2s"
  sensitive   = false
}

variable "aks_min_count" {
  description = "Kubernetes Min Node Count"
  type        = number
  default     = 1
  sensitive   = false
}

variable "aks_max_count" {
  description = "Kubernetes Max Node Count"
  type        = number
  default     = 3
  sensitive   = false
}


locals {
  workload_affix = "icecream"
  aks_namespace  = "default"
}

### Client Config Data Source ###

data "azurerm_client_config" "current" {
}

### Resources ###

# Group

resource "azurerm_resource_group" "default" {
  name     = "rg-${local.workload_affix}"
  location = var.location
}


# Log Analytics Workspace

resource "azurerm_log_analytics_workspace" "default" {
  name                = "log-${local.workload_affix}"
  location            = azurerm_resource_group.default.location
  resource_group_name = azurerm_resource_group.default.name
  sku                 = "PerGB2018"
}


# AKS

resource "azurerm_kubernetes_cluster" "default" {
  name                      = "aks-${local.workload_affix}"
  location                  = azurerm_resource_group.default.location
  resource_group_name       = azurerm_resource_group.default.name
  dns_prefix                = "cluster"
  node_resource_group       = "k8s-aks-${local.workload_affix}"
  automatic_channel_upgrade = var.aks_auto_upgrade_channel
  kubernetes_version        = var.aks_version

  oms_agent {
    log_analytics_workspace_id = azurerm_log_analytics_workspace.default.id
  }

  default_node_pool {
    name                = local.aks_namespace
    vm_size             = var.aks_vm_size
    enable_auto_scaling = true
    min_count           = var.aks_min_count
    max_count           = var.aks_max_count
  }

  identity {
    type = "SystemAssigned"
  }

  lifecycle {
    ignore_changes = [
      default_node_pool[0].node_count
    ]
  }
}

# Update Container insights to enable metrics
# https://docs.microsoft.com/en-us/azure/azure-monitor/containers/container-insights-update-metrics
resource "azurerm_role_assignment" "example" {
  scope                = azurerm_kubernetes_cluster.default.id
  role_definition_name = "Monitoring Metrics Publisher"
  principal_id         = azurerm_kubernetes_cluster.default.oms_agent[0].oms_agent_identity[0].object_id
}


# Container Insights

resource "azurerm_log_analytics_solution" "default" {
  solution_name         = "ContainerInsights"
  location              = azurerm_resource_group.default.location
  resource_group_name   = azurerm_resource_group.default.name
  workspace_resource_id = azurerm_log_analytics_workspace.default.id
  workspace_name        = azurerm_log_analytics_workspace.default.name

  plan {
    publisher = "Microsoft"
    product   = "OMSGallery/ContainerInsights"
  }
}


# Azure Monitor Diagnostic Settings

resource "azurerm_monitor_diagnostic_setting" "aks" {
  name                       = "Kubernetes Logs"
  target_resource_id         = azurerm_kubernetes_cluster.default.id
  log_analytics_workspace_id = azurerm_log_analytics_workspace.default.id

  log {
    category = "kube-apiserver"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  log {
    category = "kube-audit"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  log {
    category = "kube-audit-admin"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  log {
    category = "kube-controller-manager"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  log {
    category = "kube-scheduler"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  log {
    category = "cluster-autoscaler"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  log {
    category = "cloud-controller-manager"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  log {
    category = "guard"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  log {
    category = "csi-azuredisk-controller"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  log {
    category = "csi-azurefile-controller"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  log {
    category = "csi-snapshot-controller"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }

  metric {
    category = "AllMetrics"
    enabled  = true

    retention_policy {
      days    = 7
      enabled = true
    }
  }
}


### Output ###

output "az_get_credentials" {
  value     = "az aks get-credentials -n ${azurerm_kubernetes_cluster.default.name} -g ${azurerm_resource_group.default.name}"
  sensitive = false
}
