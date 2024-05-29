package sdk

import (
	"encoding/json"

	"github.com/infisical/go-sdk/internal/cinterface"
)

type CommandRunnerInterface interface {
	RunCommand(command Command) (string, error)
}

type CommandRunner struct {
	client cinterface.ClientPointer
	lib    cinterface.InfisicalLibrary
}

func NewCommandRunner(client cinterface.ClientPointer, lib cinterface.InfisicalLibrary) *CommandRunner {
	return &CommandRunner{
		client: client,
		lib:    lib,
	}
}

func (c *CommandRunner) RunCommand(command Command) (string, error) {
	commandJSON, err := json.Marshal(command)
	if err != nil {
		return "", err
	}

	responseStr, err := c.lib.RunCommand(string(commandJSON), c.client)
	if err != nil {
		return "", err
	}

	return responseStr, nil
}
