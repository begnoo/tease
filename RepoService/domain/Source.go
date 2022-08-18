package domain

import (
	"time"

	"gorm.io/gorm"
)

type BranchHead struct {
	gorm.Model
	Name     string
	Sha1     string
	SourceID int
}

type Source struct {
	gorm.Model
	Name          string
	Owner         string
	Collabarators []Collabarator
	Heads         []BranchHead
	Visability    bool
	Initialized   bool
}

type Collabarator struct {
	gorm.Model
	Name            string
	ReactedToInvite bool
	AcceptedInvite  bool
	ExpiersAt       time.Time
	SourceID        int
}

type TreeBlob struct {
}

type FileBlob struct {
}

type CommitBlob struct {
}
