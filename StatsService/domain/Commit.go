package domain

import (
	"go.mongodb.org/mongo-driver/bson/primitive"
)

type Commit struct {
	ID        primitive.ObjectID `bson:"_id,omitempty" json:"id"`
	CreatedAt int                `mapper:"CreatedAt" bson:"created_at" json:"created_at"`
	Added     int                `mapper:"Added" bson:"added" json:"added"`
	Deleted   int                `mapper:"Deleted" bson:"deleted" json:"deleted"`
	Owner     string             `mapper:"Owner" bson:"owner" json:"owner"`
	User      string             `mapper:"User" bson:"user" json:"user"`
	Source    string             `mapper:"Source" bson:"source" json:"source"`
	Sha       string             `mapper:"Sha" bson:"sha" json:"sha"`
}

type CommitCountByUser struct {
	User    string `json:"user" bson:"_id"`
	Count   int    `json:"count" bson:"count"`
	Added   int    `json:"added" bson:"added"`
	Deleted int    `json:"deleted" bson:"deleted"`
}

type CommitCountByDay struct {
	ID      string `bson:"_id,omitempty" json:"id"`
	Count   int    `json:"count" bson:"count"`
	Added   int    `json:"added" bson:"added"`
	Deleted int    `json:"deleted" bson:"deleted"`
}
