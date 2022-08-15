package jobs

import (
	"RepoService/di"
	"fmt"

	"github.com/jasonlvhit/gocron"
)

func task() {
	fmt.Println("Deleting expiered invitations.")
	collabService := di.InitializeCollabService()
	collabService.DeleteExpieredInvitations()
}

func InitTask() {
	s := gocron.NewScheduler()
	s.Every(5).Seconds().Do(task)
	<-s.Start()
}
