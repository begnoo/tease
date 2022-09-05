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
	Visability    bool      `mapper:"Visability"`
	Initialized   bool      `mapper:"Initialized"`
	Collabarators []Collabarator
}

type Collabarator struct {
	gorm.Model
	ID              int       `mapper:"ID"`
	Name            string    `mapper:"Name"`
	SourceName      string    `mapper:"SourceName"`
	From            string    `mapper:"From"`
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
