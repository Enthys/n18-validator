package main

import (
	"encoding/xml"
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/harry1453/go-common-file-dialog/cfd"
	"github.com/harry1453/go-common-file-dialog/cfdutil"
	"github.com/sqweek/dialog"
)

func main() {
	mainFile := promptSelectMainFile()
	documentNumbers := InterceptFiles(mainFile, promptSelectPreviousFiles())
	if dialog.Message(generateConfirmationMsg(documentNumbers)).Title("Confirm Removal").YesNo() {
		audit := RemoveInterceptedNumbers(mainFile, documentNumbers)
		audit = RecalculateRefunds(audit)
		generateFileFromAudit(mainFile, audit)
	}
}

func generateFileFromAudit(fileName string, audit Audit) {
	newReportName := strings.Replace(fileName, ".xml", "_clear.xml", 1)

	newFile, _ := os.Create(newReportName)
	defer newFile.Close()

	xmlInBytes, _ := xml.Marshal(audit)

	header := strings.Replace(xml.Header, "UTF-8", "WINDOWS-1251", 1)
	newFile.WriteString(header + string(xmlInBytes))
}

func generateConfirmationMsg(documentNumbers []string) string {
	message := fmt.Sprintf("Will remove the following(%v) records from both orders and refunds: \n", len(documentNumbers))
	documentNumberChunks := chunkBy(documentNumbers, 7)
	for _, documentNumberChunk := range documentNumberChunks {
		message += strings.Join(documentNumberChunk, ", ") + "\n"
	}

	return message
}

func chunkBy(items []string, chunkSize int) (chunks [][]string) {
	for chunkSize < len(items) {
		items, chunks = items[chunkSize:], append(chunks, items[0:chunkSize:chunkSize])
	}

	return append(chunks, items)
}

func promptSelectMainFile() string {
	result, err := cfdutil.ShowOpenFileDialog(cfd.DialogConfig{
		Title: "Select files with which to intercept",
		Role:  "OpenMainAuditFile",
		FileFilters: []cfd.FileFilter{
			{
				DisplayName: "",
				Pattern:     "*.xml",
			},
		},
		SelectedFileFilterIndex: 2,
		FileName:                "",
		DefaultExtension:        "txt",
	})
	if err != nil {
		log.Fatal(err)
	}

	return result
}

func promptSelectPreviousFiles() []string {
	result, err := cfdutil.ShowOpenMultipleFilesDialog(cfd.DialogConfig{
		Title: "Select files with which to intercept",
		Role:  "OpenFileExample",
		FileFilters: []cfd.FileFilter{
			{
				DisplayName: "",
				Pattern:     "*.xml",
			},
		},
		SelectedFileFilterIndex: 2,
		FileName:                "",
		DefaultExtension:        "txt",
	})
	if err != nil {
		log.Fatal(err)
	}

	return result
}
