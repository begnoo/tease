package domain

import (
	"go.mongodb.org/mongo-driver/bson/primitive"
)

type Commit struct {
	ID        primitive.ObjectID `bson:"_id,omitempty" json:"id"`
	CreatedAt int                `bson:"created_at" json:"created_at"`
	Added     int                `bson:"added" json:"added"`
	Deleted   int                `bson:"deleted" json:"deleted"`
	Owner     string             `bson:"owner" json:"owner"`
	User      string             `bson:"user" json:"user"`
	Source    string             `bson:"source" json:"source"`
	Sha       string             `bson:"sha" json:"sha"`
}

type CommitCountByUser struct {
	User    string `json:"user" bson:"_id"`
	Count   int    `json:"count" bson:"count"`
	Added   int    `json:"added" bson:"added"`
	Deleted int    `json:"deleted" bson:"deleted"`
}
