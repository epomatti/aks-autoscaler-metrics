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
  default     = "brazilsouth"
  sensitive   = false
  description = "Set .auto.tfvars to a location near you"
}

locals {
  app_affix     = "icecream-app"
  password      = "DemoPassword!123"
  aks_namespace = "default"
}

### Resources ###

resource "azurerm_resource_group" "default" {
  name     = "rg-${local.app_affix}"
  location = var.location
}

resource "azurerm_kubernetes_cluster" "default" {
  name                = "aks-${local.app_affix}"
  location            = azurerm_resource_group.default.location
  resource_group_name = azurerm_resource_group.default.name
  dns_prefix          = "aks"
  node_resource_group = "k8s-aks-${local.app_affix}"

  default_node_pool {
    name       = local.aks_namespace
    node_count = 1
    vm_size    = "Standard_D2_v2"
  }

  identity {
    type = "SystemAssigned"
  }
}


output "az_get_credentials" {
  value     = "az aks get-credentials -n ${azurerm_kubernetes_cluster.default.name} -g ${azurerm_resource_group.default.name}"
  sensitive = false
}
