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
	Description   string    `mapper:"Description"`
	Owner         string    `mapper:"Owner"`
	Collabarators []Collabarator
	Visability    bool `mapper:"Visability"`
	Initialized   bool `mapper:"Initialized"`
}

type Collabarator struct {
	gorm.Model
	ID              int       `mapper:"ID"`
	Name            string    `mapper:"Name"`
	ReactedToInvite bool      `mapper:"ReactedToInvite"`
	AcceptedInvite  bool      `mapper:"AcceptedInvite"`
	ExpiersAt       time.Time `mapper:"ExpiersAt"`
	SourceID        int       `mapper:"SourceID"`
}

type TreeBlob struct {
}

type FileBlob struct {
}

type CommitBlob struct {
}
