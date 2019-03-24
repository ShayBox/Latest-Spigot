package main

import (
	"io"
	"net/http"
	"os"
	"os/exec"
)

func main() {
	println("Deleting Build directory (if it exists)")
	os.RemoveAll("Build")

	println("Creating Build directory")
	os.Mkdir("Build", os.ModePerm)

	println("Download BuildTools.jar to Build directory")
	DownloadFile("Build/BuildTools.jar", "https://hub.spigotmc.org/jenkins/job/BuildTools/lastSuccessfulBuild/artifact/target/BuildTools.jar")

	println("Building Spigot/Craftbukkit")
	cmd := exec.Command("java", "-Xms512M", "-jar", "BuildTools.jar")
	cmd.Dir = "./Build"
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		panic(err)
	}
	cmd.Wait()
}

// DownloadFile will download a url to a local file. It's efficient because it will
// write as it downloads and not load the whole file into memory.
func DownloadFile(filepath string, url string) error {

	// Get the data
	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	// Create the file
	out, err := os.Create(filepath)
	if err != nil {
		return err
	}
	defer out.Close()

	// Write the body to file
	_, err = io.Copy(out, resp.Body)
	return err
}
