package request

type Commit struct {
	CreatedAt int    `mapper:"CreatedAt" json:"created_at" validate:"required"`
	Added     int    `mapper:"Added" json:"added" validate:"required"`
	Deleted   int    `mapper:"Deleted" json:"deleted" validate:"required"`
	Owner     string `mapper:"Owner" json:"owner" validate:"required,email"`
	User      string `mapper:"User" json:"user" validate:"required,email"`
	Source    string `mapper:"Source" json:"source" validate:"required"`
	Sha       string `mapper:"Sha" json:"sha" validate:"required"`
	Branch    string `mapper:"Branch" json:"branch" validate:"required"`
}

type Commits struct {
	Items []Commit `json:"items"`
}
