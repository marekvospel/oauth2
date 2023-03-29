
job "oauth-nomad" {

  group "database" {
    count = 1

    network {
      mode = "bridge"
      port "db" {
        to = 5432
      }
    }

    service {
      name = "oauth-postgres"
      port = "db"

      connect {
        sidecar_service {}
      }
    }

    task "postgres" {
      driver = "docker"

      config {
        image = "postgres:latest"
      }

      resources {
        cpu = 300
        memory = 1536
      }

      env {
        POSTGRES_PASSWORD = "mysecretpassword"
      }
    }

  }

  group "backend" {
    network {
      mode = "bridge"
      port "http" {
        to = 80
      }
    }

    service {
      name = "oauth-backend"
      port = "http"

      connect {
        sidecar_service {
          proxy {
            upstreams {
              destination_name = "oauth-postgres"
              local_bind_port = 5432
            }
          }
        }
      }
    }

    task "backend" {
      driver = "docker"

      config {
        image = "ghcr.io/marekvospel/oauth2-backend"
      }

      resources {
        cpu = 300
        memory = 1536
      }

      env {
        ROCKET_DATABASES = "{ sea_orm = { url = \"postgres://postgres:mysecretpassword@localhost:5432\" } }"
        ROCKET_ADDRESS = "0.0.0.0"
        ROCKET_PORT = "80"
      }

      lifecycle {
        hook = "prestart"
        sidecar = true
      }
    }
  }

}
