package responses

import "time"

type Source struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Owner       string `json:"owner"`
	Visability  bool   `json:"visability"`
	Initialized bool   `json:"initialized"`
	// Collabarators []Collabarator `json:"collabarators"`
}

type Collabarator struct {
	ID              string    `json:"id"`
	Name            string    `json:"name"`
	ReactedToInvite bool      `json:"reactedToInvite"`
	AcceptedInvite  bool      `json:"acceptedInvite"`
	ExpiersAt       time.Time `json:"expiersAt"`
}
