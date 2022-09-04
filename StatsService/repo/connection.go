package repo

import (
	"fmt"
	"log"
	"os"

	"context"

	"github.com/joho/godotenv"
	"go.mongodb.org/mongo-driver/event"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
)

var client *mongo.Client = nil

func ProvideConnection() *mongo.Client {

	if client != nil {
		return client
	}

	err := godotenv.Load(".env")
	if err != nil {
		panic(err)
	}

	dsn := fmt.Sprintf("mongodb://%s:%s@%s:%s/?authSource=admin",
		os.Getenv("DB_USER"),
		os.Getenv("DB_PASS"),
		os.Getenv("DB_HOST"),
		os.Getenv("DB_PORT"),
	)
	cmdMonitor := &event.CommandMonitor{
		Started: func(_ context.Context, evt *event.CommandStartedEvent) {
			log.Print(evt.Command)
		},
	}

	mongo_client, err := mongo.Connect(context.TODO(), options.Client().ApplyURI(dsn).SetMonitor(cmdMonitor))
	if err != nil {
		panic(err)
	}

	if err := mongo_client.Ping(context.TODO(), readpref.Primary()); err != nil {
		panic(err)
	}

	client = mongo_client
	return mongo_client
}
