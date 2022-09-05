package repo

import (
	"StatsService/domain"
	"context"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type CloneRepo struct {
	db *mongo.Collection
}

func ProvideCloneRepo(client *mongo.Client) CloneRepo {
	return CloneRepo{
		db: client.Database("tease_stats").Collection("clones"),
	}
}

func (r *CloneRepo) Create(commit domain.Clone) (*mongo.InsertOneResult, error) {
	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	res, err := r.db.InsertOne(ctx, commit)

	return res, err
}

func (repo *CloneRepo) ReadBySource(owner, name string) (*[]domain.Clone, error) {

	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	var commits []domain.Clone
	opts := options.Find()
	opts.SetSort(bson.D{{Key: "created_at", Value: -1}})
	cursor, err := repo.db.Find(ctx, bson.M{"owner": owner, "source": name}, opts)

	if err != nil {
		return nil, err
	}
	defer cursor.Close(ctx)

	for cursor.Next(ctx) {
		commit := new(domain.Clone)
		if err := cursor.Decode(commit); err != nil {
			return nil, err
		}

		commits = append(commits, *commit)
	}

	return &commits, err
}
