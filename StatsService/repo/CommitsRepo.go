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

func (r *CommitRepo) CreateCommits(commits []domain.Commit) (*mongo.InsertManyResult, error) {
	newValue := make([]interface{}, len(commits))
	for i, v := range commits {
		newValue[i] = v
	}

	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	res, err := r.db.InsertMany(ctx, newValue)

	return res, err
}

func (r *CommitRepo) UpdateCommits(commits []domain.Commit) (*mongo.UpdateResult, error) {

	objectShas := make([]string, len(commits))
	for i, v := range commits {
		objectShas[i] = v.Sha
	}

	filter := bson.D{{Key: "sha", Value: bson.D{
		{Key: "$in", Value: objectShas}},
	}}

	update := bson.D{{Key: "$set", Value: bson.D{
		{Key: "branch", Value: commits[0].Branch}},
	}}

	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	res, err := r.db.UpdateMany(ctx, filter, update)

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

func (repo *CommitRepo) ReadBySourceGroupByDay(owner, name string) (*[]domain.CommitCountByDay, error) {

	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	matchStage := bson.D{{
		Key: "$match", Value: bson.D{
			{Key: "owner", Value: owner},
			{Key: "source", Value: name},
			{Key: "branch", Value: "master"},
		},
	}}
	groupStage := bson.D{{
		Key: "$group",
		Value: bson.D{
			{Key: "_id", Value: bson.D{
				{Key: "$dateToString", Value: bson.D{
					{Key: "format", Value: "%d-%m-%Y"},
					{Key: "date", Value: bson.D{
						{Key: "$toDate", Value: bson.D{
							{Key: "$multiply", Value: bson.A{1000, "$created_at"}},
						}},
					}},
				}},
			}},
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

	var commits []domain.CommitCountByDay
	cursor, err := repo.db.Aggregate(ctx, mongo.Pipeline{matchStage, groupStage})

	if err != nil {
		return nil, err
	}
	defer cursor.Close(ctx)

	for cursor.Next(ctx) {
		fmt.Printf("%+v", cursor.Current)
		commit := new(domain.CommitCountByDay)
		if err := cursor.Decode(commit); err != nil {
			return nil, err
		}

		commits = append(commits, *commit)
	}

	return &commits, err
}

func (repo *CommitRepo) ReadBySourceGroupByCollabAndDay(owner, name string) (*[]domain.CommitCountByUserAndDay, error) {

	ctx, cancel := context.WithTimeout(context.Background(), mongoQueryTimeout)
	defer cancel()

	matchStage := bson.D{{
		Key: "$match", Value: bson.D{
			{Key: "owner", Value: owner},
			{Key: "source", Value: name},
			{Key: "branch", Value: "master"},
		},
	}}

	groupStage := bson.D{{
		Key: "$group",
		Value: bson.D{
			{Key: "_id", Value: bson.D{
				{Key: "date", Value: bson.D{
					{Key: "$dateToString", Value: bson.D{
						{Key: "format", Value: "%d-%m-%Y"},
						{Key: "date", Value: bson.D{
							{Key: "$toDate", Value: bson.D{
								{Key: "$multiply", Value: bson.A{1000, "$created_at"}},
							}},
						}},
					}},
				}},
				{Key: "user", Value: "$user"},
			}},
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

	var commits []domain.CommitCountByUserAndDay
	cursor, err := repo.db.Aggregate(ctx, mongo.Pipeline{matchStage, groupStage})

	if err != nil {
		return nil, err
	}
	defer cursor.Close(ctx)

	for cursor.Next(ctx) {
		fmt.Printf("%+v", cursor.Current)
		commit := new(domain.CommitCountByUserAndDay)
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
			{Key: "branch", Value: "master"},
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
	cursor, err := repo.db.Find(ctx, bson.M{"user": user, "branch": "master"}, opts)

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
