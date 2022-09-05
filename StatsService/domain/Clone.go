package domain

import "go.mongodb.org/mongo-driver/bson/primitive"

type Clone struct {
	ID        primitive.ObjectID `bson:"_id,omitempty" json:"id"`
	CreatedAt int                `bson:"created_at" json:"created_at"`
	Owner     string             `bson:"owner" json:"owner"`
	Source    string             `bson:"source" json:"source"`
	User      string             `bson:"user" json:"user"`
}
