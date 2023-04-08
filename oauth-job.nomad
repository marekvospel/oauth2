
job "oauth" {

  group "oauth-database" {
    count = 1

    network {
      mode = "bridge"

      port "postgres" {
        static = "5432"
      }
    }

    service {
      name = "oauth-postgres"
      port = "5432"

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

  group "oauth-api" {

    network {
      mode = "bridge"

      port "api" {
        static = 80
      }
    }

    service {
      name = "oauth-api"
      port = "80"

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

    task "oauth-api" {
      driver = "docker"

      config {
        image = "ghcr.io/marekvospel/oauth2-backend"
      }

      resources {
        cpu = 300
        memory = 1536
      }

      env {
        ROCKET_DATABASES = "{ sea_orm = { url = \"postgres://postgres:mysecretpassword@127.0.0.1:5432\" } }"
        ROCKET_ADDRESS = "0.0.0.0"
        ROCKET_PORT = "80"
      }
    }
  }

}

