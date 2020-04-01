variable "REGION" {
  description = "Azure region"
  default     = "westeurope"
}

variable "RESOURCE_GROUP_PRODUCTS" {
  description = "Products resource group name"
  default     = "mhra-products-development"
}

variable "CLIENT_ID" {
  description = "Service Principal Client ID"
}

variable "CLIENT_SECRET" {
  description = "Service Principal Client Secret"
}

variable "ENVIRONMENT" {
  description = "Environment name"
  default     = "dev"
}
