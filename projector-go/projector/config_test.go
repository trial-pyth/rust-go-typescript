package projector_test

import (
	"reflect"
	"testing"

	"github.com/trial-pyth/rust-go-typescript/projector-go/projector"
)


func TestConfigPrint(t *testing.T) {
	opts := projector.Opts{
		Args:   []string{},
		Config: "",
		Pwd:    "",
	}

	config, err := projector.NewConfig(&opts)
	if err != nil {
		t.Errorf("expected to get no error %v", err)
	}

	if config.Operation != projector.Print {
		t.Errorf("operation expected was print but got %v", config.Operation)

	}

	if !reflect.DeepEqual([]string{}, config.Args) {
		t.Errorf("expected args to be an empty string array but got %+v", config.Args)
	}
}

func TestConfigPrintKey(t *testing.T) {
	opts := projector.Opts{
		Args:   []string{"foo"},
	}

	config, err := projector.NewConfig(&opts)
	if err != nil {
		t.Errorf("expected to get no error %v", err)
	}

	if config.Operation != projector.Print {
		t.Errorf("operation expected was print but got %v", config.Operation)

	}

	if !reflect.DeepEqual([]string{"foo"}, config.Args) {
		t.Errorf("expected args to be an empty string array but got %+v", config.Args)
	}
}

func TestConfigAdd(t *testing.T) {
	opts := projector.Opts{
		Args:   []string{"add", "foo", "bar"},
		Config: "",
		Pwd:    "",
	}

	config, err := projector.NewConfig(&opts)
	if err != nil {
		t.Errorf("expected to get no error %v", err)
	}

	if config.Operation != projector.Add {
		t.Errorf("operation expected was add but got %v", config.Operation)
	}

	if config.Args[0] != "foo" || config.Args[1] != "bar" {
		t.Errorf("expected arguments to equal {'foo', 'bar'} but got %+v", config.Args)
	}
}

func TestConfigAddKeyValue(t *testing.T) {
	opts := projector.Opts{
		Args:   []string{"add", "foo", "bar"},
		Config: "",
		Pwd:    "",
	}

	config, err := projector.NewConfig(&opts)
	if err != nil {
		t.Errorf("expected to get no error %v", err)
	}

	if config.Operation != projector.Add {
		t.Errorf("operation expected was add but got %v", config.Operation)
	}

	if config.Args[0] != "foo" || config.Args[1] != "bar" {
		t.Errorf("expected arguments to equal {'foo', 'bar'} but got %+v", config.Args)
	}
}
