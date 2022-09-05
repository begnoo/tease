package request

type Clone struct {
	CreatedAt int    `bson:"created_at" json:"created_at"`
	Owner     string `bson:"owner" json:"owner"`
	Source    string `bson:"source" json:"source"`
	User      string `bson:"user" json:"user"`
}
