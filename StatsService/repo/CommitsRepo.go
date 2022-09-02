package repo

import (
	"StatsService/domain"
	"context"
	"fmt"
	"time"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

const (
	mongoQueryTimeout = 10 * time.Second
)

type CommitRepo struct {
	db *mongo.Collection
}

func ProvideCommitRepo(client *mongo.Client) CommitRepo {
	return CommitRepo{
		db: client.Database("tease_stats").Collection("commits"),
	}
}

func (r *CommitRepo) Create(commit domain.Commit) (*mongo.InsertOneResult, error) {
	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	res, err := r.db.InsertOne(ctx, commit)

	return res, err
}

func (r *CommitRepo) CreateCommits(commits []interface{}) (*mongo.InsertManyResult, error) {
	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	res, err := r.db.InsertMany(ctx, commits)

	return res, err
}

func (repo *CommitRepo) ReadBySource(owner, name string) (*[]domain.Commit, error) {

	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	var commits []domain.Commit
	opts := options.Find()
	opts.SetSort(bson.D{{Key: "created_at", Value: -1}})
	cursor, err := repo.db.Find(ctx, bson.M{"owner": owner, "source": name}, opts)

	if err != nil {
		return nil, err
	}
	defer cursor.Close(ctx)

	for cursor.Next(ctx) {
		commit := new(domain.Commit)
		if err := cursor.Decode(commit); err != nil {
			return nil, err
		}

		commits = append(commits, *commit)
	}

	return &commits, err
}

func (repo *CommitRepo) ReadBySourceGroupByUser(owner, name string) (*[]domain.CommitCountByUser, error) {

	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	matchStage := bson.D{{
		Key: "$match", Value: bson.D{
			{Key: "owner", Value: owner},
			{Key: "source", Value: name},
		},
	}}
	groupStage := bson.D{{
		Key: "$group",
		Value: bson.D{
			{Key: "_id", Value: "$user"},
			{Key: "count", Value: bson.D{
				{Key: "$count", Value: bson.D{}},
			}},
			{Key: "added", Value: bson.D{
				{Key: "$sum", Value: "$added"},
			}},
			{Key: "deleted", Value: bson.D{
				{Key: "$sum", Value: "$deleted"},
			}},
		},
	}}

	var commits []domain.CommitCountByUser
	opts := options.Find()
	opts.SetSort(bson.D{{Key: "created_at", Value: -1}})
	cursor, err := repo.db.Aggregate(ctx, mongo.Pipeline{matchStage, groupStage})
	if err != nil {
		return nil, err
	}
	defer cursor.Close(ctx)

	for cursor.Next(ctx) {
		commit := new(domain.CommitCountByUser)
		if err := cursor.Decode(commit); err != nil {
			return nil, err
		}

		commits = append(commits, *commit)
	}

	return &commits, err
}

func (repo *CommitRepo) ReadByUser(user string) (*[]domain.Commit, error) {

	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	var commits []domain.Commit
	opts := options.Find()
	opts.SetSort(bson.D{{Key: "created_at", Value: -1}})
	cursor, err := repo.db.Find(ctx, bson.M{"user": user}, opts)

	if err != nil {
		return nil, err
	}
	defer cursor.Close(ctx)

	for cursor.Next(ctx) {
		fmt.Printf("bla\n")
		commit := new(domain.Commit)
		if err := cursor.Decode(commit); err != nil {
			return nil, err
		}

		commits = append(commits, *commit)
	}

	return &commits, err
}
