### main Resource configuration
# Original template from https://github.com/kube-hetzner/terraform-hcloud-kube-hetzner/blob/master/kube.tf.example

# https://github.com/kube-hetzner/terraform-hcloud-kube-hetzner/blob/master/docs/terraform.md
module "kube-hetzner" {
  source = "kube-hetzner/kube-hetzner/hcloud"
  # https://registry.terraform.io/modules/kube-hetzner/kube-hetzner/hcloud
  version = "2.17.1"
  providers = {
    hcloud = hcloud
  }
  hcloud_token = var.hcloud_token

  create_kubeconfig = false # do not export local kubeconfig file implicitely
  export_values = true # do not export local files 
  create_kustomization = true # do not create local file for kustomization backup
  # This does not protect deletion from Terraform itself, only though the Hetzner UI interface
  enable_delete_protection = {
    floating_ip   = true
    load_balancer = true
    volume        = true
  }

  microos_x86_snapshot_id = var.microos_x86_snapshot_id
  microos_arm_snapshot_id = var.microos_arm_snapshot_id

  ssh_port = 2220 # defualt: 22
  ssh_public_key = file(var.ssh_public_key)
  ssh_private_key = file(var.ssh_private_key)

  cluster_name = "k3s"
  use_cluster_name_in_node_name = true
  automatically_upgrade_k3s = false
  # NOTE: checkout also longhorn_values > node-down-pod-deletion-policy
  system_upgrade_enable_eviction = true
  system_upgrade_use_drain = true
  initial_k3s_channel = "stable" # "v1.32" # "stable"
  allow_scheduling_on_control_plane = false

  automatically_upgrade_os = false # NOTE: must be turned off for single control node setup.
  kured_options = {
    "reboot-days": "su,mo,tu,we,th,fr,sa",
    "start-time": "9pm",
    "end-time": "4pm",
    "time-zone": "America/Chicago",
  }

  network_region = var.network_location[0].zone 
  # DNS provided by Hetzner https://docs.hetzner.com/dns-console/dns/general/recursive-name-servers/.
  dns_servers = [
    "1.1.1.1",  # Cloudflare
    "8.8.8.8",  # Google DNS
    "2606:4700:4700::1111"  # IPv6 Cloudflare DNS
    # "185.12.64.1", # Hetzner DNS
    # "2a01:4ff:ff00::add:1", # Hetzner DNS
  ]
  extra_firewall_rules = [
  ]

  # https://www.hetzner.com/cloud/load-balancer
  load_balancer_type     = "lb11"
  load_balancer_location = "fsn1"
  enable_klipper_metal_lb = "false"
  # Disables the public network of the load balancer. (default: false).
  # load_balancer_disable_public_network = true

  disable_kube_proxy = true # replace 'kube-proxy' with 'cilium'
  disable_network_policy = true
  ingress_controller = "none"
  cni_plugin = "cilium"
  cilium_version = "v1.17.4"  # upgdated from "v1.16.6"
  cilium_routing_mode = "native"
  # NOTE: if Cilium UI enabled it can be accessed using ssh tunnel
  cilium_hubble_enabled = false
  # Configures the list of Hubble metrics to collect.
  # cilium_hubble_metrics_enabled = [] # specified in the overriding custom cilium-values.yaml file
  # https://github.com/cilium/cilium/blob/main/install/kubernetes/cilium/values.yaml
  cilium_values = local.helm_values_file["cilium"]

  # for production HA kubernetes: 3 control nodes + 2 agent nodes
  control_plane_nodepools = [
    {
      name        = "control-plane",
      count       = 1 # NOTE: set to 3 control nodes for HA and allowing OS upgrades
      server_type = var.instance_size.small,
      location    = var.network_location[0].region[0],
      placement_group = "controller"
      # labels      = concat(local.label.control_plane, ["node.longhorn.io/create-default-disk=config"]),
      labels      = local.label.control_plane,
      taints      = [],
      # kubelet_args = ["kube-reserved=cpu=250m,memory=1500Mi,ephemeral-storage=1Gi", "system-reserved=cpu=250m,memory=300Mi"]

      # Enable automatic backups via Hetzner (default: false)
      # backups = true
    },
    # {
    #   name        = "control-plane-arm",
    #   count       = 0 
    #   server_type = var.instance_size.small_arm,
    #   location    = var.network_location[0].region[0],
    #   placement_group = "controller"
    #   # labels      = concat(local.label.control_plane, ["node.longhorn.io/create-default-disk=config"]),
    #   labels      = local.label.control_plane,
    #   taints      = [],
    # }
  ]

  # node type per pool > # nodes
  # NOTE: to remove nodes, drain them first from any kubernetes workloads (`kubectl drain ...`)
  agent_nodepools = [
    {
      name        = "worker-small",
      count       = 0, 
      server_type = var.instance_size.small,
      location    = var.network_location[0].region[0],
      placement_group = "worker"
      labels      = concat(local.label.worker, ["node.longhorn.io/create-default-disk=config"]),
      taints      = [],
      longhorn_volume_size = 10
    },

    {
      name        = "worker-medium",
      count       = 2
      server_type = var.instance_size.medium,
      location    = var.network_location[0].region[0],
      placement_group = "worker"
      labels      = concat(local.label.worker, ["node.longhorn.io/create-default-disk=config"]),
      taints      = [],
      longhorn_volume_size = 10
    },

    # {
    #   name        = "worker-large",
    #   count       = 0
    #   server_type = var.instance_size.large,
    #   location    = var.network_location[0].region[0],
    #   placement_group = "worker"
    #   labels      = concat(local.label.worker, ["node.longhorn.io/create-default-disk=config"]),
    #   taints      = [],
    #   longhorn_volume_size = 10
    # },

    # {
    #   # follow link to set values for reserved storage: https://gist.github.com/ifeulner/d311b2868f6c00e649f33a72166c2e5b
    #   name        = "example-longhorn-storage-volume",
    #   count       = 0
    #   server_type = var.instance_size.small,
    #   location    = var.network_location[0].region[0],
    #   placement_group = "worker"
    #   labels      = concat(local.label.worker, ["node.longhorn.io/create-default-disk=config"]),  # use config in annotations instead of single option label (annotations must be set using `kubectl annotate node ...`  command)
    #   taints      = [],
      
    #   # option 1 - longhorn on local machine storage: Fast storage (e.g. for DB app usage)
    #   # longhorn_volume_size = 0
    #   # option 2 - longhorn on network storage: slower storage using Hetzner Cloud Volume network block storage instead of machine's local storage (e.g. good for backup usage, any low IOPS requirement)
    #   longhorn_volume_size = 10
    # },
  ]

  autoscaler_nodepools = [
  #   {
  #     name        = "autoscaled-small"
  #     server_type = var.instance_size.small
  #     location    = var.network_location[0].region[0],
  #     min_nodes   = 0
  #     max_nodes   = 2
  #     labels      = {
  #       "node.kubernetes.io/role": "peak-workloads" # convention labels (doesn't seem used by )
  #     }
  #     taints      = [
  #       {
  #         key= "node.kubernetes.io/role"
  #         value= "peak-workloads"
  #         effect= "NoExecute"
  #       }
  #     ]
  #     # kubelet_args = ["kube-reserved=cpu=250m,memory=1500Mi,ephemeral-storage=1Gi", "system-reserved=cpu=250m,memory=300Mi"]
  #  }
  ]

  # local k3s basic storage class
  enable_local_storage = true
  # cloud network storage: 
  disable_hetzner_csi = false # Hetzner Cloud Volumes (block storage) as Kubernetes storage class; when enabled volumes can be automatially created on Hetzner Cloud and associated with servers.

  # enable longhorn and dependency drivers
  enable_iscsid = true
  enable_longhorn = true # add Longhorn as storage class in kuberenetes
  longhorn_version = "v1.9.0" # previous "v1.8.0", "v1.7.2"
  # TODO: fix[requires PR]: the module doesn't install all required dependeices on control nodes and prevents Longhorn from being able to create disks and schedule on cotnrol nodes (thus leaving network longhorn volumes only for worker nodes )
  longhorn_helmchart_bootstrap = false # if to run on control-plane nodes too 
  longhorn_fstype = "ext4" # "xfs"
  longhorn_replica_count = 2 # defaults to 3
  # effects of options: only run on labelled nodes; # adjust to autoscaler if active in the cluster; /var/lib/longhorn local storage path (NOTE: /mnt/longhorn is the default for Hetzner cloud volume mount in kube-hetzner module); cleanup & prevent volume locks by setting policy: ensure pod is moved to an healthy node if current node is down;
  longhorn_values = local.helm_values_file["longhorn"]

  # Cert-manager for automatic TLS certificates
  enable_cert_manager = true
  cert_manager_helmchart_bootstrap = false # run on control-plane nodes too
  cert_manager_version = "v1.17.2" # TODO: v1.17.2  NOTE: downgraded from 1.16.3 to v1.15.3 resolve a bug https://github.com/cert-manager/cert-manager/issues/7337  ; checking v1.17.2 if fixed
  cert_manager_values = local.helm_values_file["cert-manager"]

  # NOTE: `extra_kustomize_deployment_commands` doesn't get to run unless there is ./extra-manifests/kustomization.yaml.tpl file this is a bug and error prone better to use post-terraform shell scripts with the kubeconfig file for connection
  # extra_kustomize_deployment_commands = <<-EOT
  # EOT

}



