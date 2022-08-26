package domain

import (
	"time"

	"gorm.io/gorm"
)

type Source struct {
	gorm.Model
	ID            int       `mapper:"ID"`
	CreatedAt     time.Time `mapper:"CreatedAt"`
	Name          string    `mapper:"Name"`
	Owner         string    `mapper:"Owner"`
	Collabarators []Collabarator
	Visability    bool `mapper:"Visability"`
	Initialized   bool `mapper:"Initialized"`
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
