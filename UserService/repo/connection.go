package repo

import (
	"UserService/domain"
	"fmt"
	"log"
	"os"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

var db *gorm.DB = nil

func ProvideConnection() *gorm.DB {
	dsn := fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=%s sslmode=%s",
		os.Getenv("DB_HOST"),
		os.Getenv("DB_USER"),
		os.Getenv("DB_PASS"),
		os.Getenv("DB_NAME"),
		os.Getenv("DB_PORT"),
		os.Getenv("SSL_MODE"),
	)
	if db == nil {
		var err error
		db, err = gorm.Open(postgres.New(postgres.Config{
			DSN:                  dsn,
			PreferSimpleProtocol: true,
		}), &gorm.Config{})

		if err != nil {
			panic("Failed to connect to users database")
		}

		initTables()

		log.Printf("Successfuly connected to db...")
	}

	return db
}

func initTables() {
	migrate("users", &domain.User{})
	migrate("roles", &domain.Role{})
	migrate("roles", &domain.Profile{})
}

func migrate(name string, domainStruct interface{}) {
	if db.AutoMigrate(domainStruct) != nil {
		log.Printf("Failed to inititialize %s table...", name)
	} else {
		log.Printf("Initialized %s table...", name)
		seed()
	}
}
