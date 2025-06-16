# Providers configuration

terraform {
  required_version = ">= 1.5.0"
  
  required_providers {
  ## TODO: add coudflare as code using cf-terraforming export tool
  #     cloudflare = {
  #       source  = "cloudflare/cloudflare"
  #       version = "~> 4.0" # Use a compatible version
  #     }

    # https://registry.terraform.io/providers/hetznercloud/hcloud/latest
    hcloud = { 
      source = "hetznercloud/hcloud"
      version = "1.49.1"
    }
    
    helm = {
      source  = "hashicorp/helm"
      version = ">= 2.17.0"
    }

  }
  
  cloud { 
    organization = "szn-app" 
    
    workspaces { 
      name = "donation-app" 
    }
  } 

}

# Hetzner Cloud Provider
provider "hcloud" {
  token = var.hcloud_token
}

# Helm Provider - allows to deploy helm pacakges
provider "helm" {
  kubernetes {
    config_path = var.kubeconfig-credentials-path
  }
}


