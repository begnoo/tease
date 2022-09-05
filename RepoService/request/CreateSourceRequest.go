package request

type CreateSourceRequest struct {
	Name        string `validate:"required,source_name" json:"name" mapper:"Name"`
	Owner       string `validate:"required,email" json:"owner" mapper:"Owner"`
	Description string `json:"description" mapper:"Description"`
	Visability  bool   `json:"visability"  mapper:"Visability"`
}
