
resource "azurerm_cdn_profile" "products" {
  name                = "spikemhraproducts${var.environment}"
  location            = var.cdn_region
  resource_group_name = var.resource_group_name
  sku                 = "Standard_Microsoft"
}

resource "azurerm_cdn_endpoint" "products" {
  name                = "spikemhraproducts${var.environment}"
  profile_name        = azurerm_cdn_profile.products.name
  location            = azurerm_cdn_profile.products.location
  resource_group_name = var.resource_group_name
  origin_host_header  = var.origin_host_name

  origin {
    name      = "spikemhraproducts${var.environment}"
    host_name = var.origin_host_name
  }
}
