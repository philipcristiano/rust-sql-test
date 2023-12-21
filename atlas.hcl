variable "destructive" {
  type    = bool
  default = false
}

env "local" {
  src = [
    "file://schema/users.hcl"
  ]

  url = "postgres://rust-sql-test@localhost/rust-sql-test?sslmode=disable"

  diff {
    skip {
      drop_schema = !var.destructive
      drop_table  = !var.destructive
    }
  }
}
