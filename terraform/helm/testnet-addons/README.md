# testnet-addons

![Version: 0.1](https://img.shields.io/badge/Version-0.1-informational?style=flat-square) ![AppVersion: 0.1.0](https://img.shields.io/badge/AppVersion-0.1.0-informational?style=flat-square)

Additional components for aptos-nodes testnet

**Homepage:** <https://aptoslabs.com/>

## Source Code

* <https://github.com/aptos-labs/aptos-core>

## Values

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| cloud | string | `"EKS"` |  |
| genesis.chain_id | string | `nil` | Aptos Chain ID |
| genesis.numValidators | string | `nil` | Number of validators deployed in this testnet |
| genesis.username_prefix | string | `"aptos-node"` | Validator username prefix, used to get genesis secrets. This should be the fullname for the aptos-node helm release |
| imageTag | string | `"devnet"` | Default image tag to use for all aptos images |
| ingress.acm_certificate | string | `nil` | The ACM certificate to install on the ingress |
| ingress.cookieDurationSeconds | int | `86400` | If stickiness is enabled, how long the session cookie should last |
| ingress.enableStickyness | bool | `true` | Whether to enable session stickiness on the underlying load balancer |
| ingress.gce_managed_certificate | string | `nil` | The GCE certificate to install on the ingress |
| ingress.gce_security_policy | string | `nil` | Security policy to apply to the backend services behind the ingress |
| ingress.gce_static_ip | string | `nil` | The GCE static IP to install on the ingress |
| ingress.loadBalancerSourceRanges | string | `nil` | List of CIDRs to accept traffic from |
| ingress.wafAclArn | string | `nil` | The ARN of the WAF ACL to install on the ingress |
| service.domain | string | `nil` | If set, the base domain name to use for External DNS |
| serviceAccount.create | bool | `true` | Specifies whether a service account should be created |
| serviceAccount.name | string | `nil` | The name of the service account to use. If not set and create is true, a name is generated using the fullname template |
| waypoint.affinity | object | `{}` |  |
| waypoint.image.pullPolicy | string | `"IfNotPresent"` | Image pull policy to use for waypoint image |
| waypoint.image.repo | string | `"joseluisq/static-web-server"` | Image repo to use for serving waypoint and genesis |
| waypoint.image.tag | string | `"2.12@sha256:a3b147754be4c38ce96189c4dbaa708c36f39dfcc043c470812c33dd53fea7d0"` | Image tag to use for serving waypoint and genesis |
| waypoint.nodeSelector | object | `{}` |  |
| waypoint.resources.requests.cpu | string | `"200m"` |  |
| waypoint.resources.requests.memory | string | `"512Mi"` |  |
| waypoint.tolerations | list | `[]` |  |

----------------------------------------------
Autogenerated from chart metadata using [helm-docs v1.13.1](https://github.com/norwoodj/helm-docs/releases/v1.13.1)