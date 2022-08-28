package responses

import "time"

type Source struct {
	ID          int       `json:"id" mapper:"ID"`
	Name        string    `json:"name" mapper:"Name"`
	Owner       string    `json:"owner" mapper:"Owner"`
	Description string    `json:"description" mapper:"Description"`
	Visability  bool      `json:"visability" mapper:"Visability"`
	Initialized bool      `json:"initialized" mapper:"Initialized"`
	CreatedAt   time.Time `json:"createdAt" mapper:"CreatedAt"`
	// Collabarators []Collabarator `json:"collabarators"`
}

type Collabarator struct {
	ID              int       `json:"id" mapper:"ID"`
	Name            string    `json:"name" mapper:"Name"`
	ReactedToInvite bool      `json:"reactedToInvite" mapper:"ReactedToInvite"`
	AcceptedInvite  bool      `json:"acceptedInvite" mapper:"AcceptedInvite"`
	ExpiersAt       time.Time `json:"expiersAt" mapper:"ExpiersAt"`
}
