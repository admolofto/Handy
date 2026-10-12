import React, { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { open } from "@tauri-apps/plugin-dialog";
import { commands } from "@/bindings";
import { useSettings } from "../../../hooks/useSettings";
import { SettingContainer } from "../../ui/SettingContainer";
import { PathDisplay } from "../../ui/PathDisplay";
import { Button } from "../../ui/Button";

interface NotesDirectoryProps {
  descriptionMode?: "tooltip" | "inline";
  grouped?: boolean;
}

export const NotesDirectory: React.FC<NotesDirectoryProps> = ({
  descriptionMode = "inline",
  grouped = false,
}) => {
  const { t } = useTranslation();
  const { getSetting, refreshSettings } = useSettings();
  const customDir = getSetting("notes_dir");
  const [notesDir, setNotesDir] = useState("");
  const [error, setError] = useState<string | null>(null);

  const loadNotesDir = useCallback(async () => {
    const result = await commands.getNotesDir();
    if (result.status === "ok") {
      setNotesDir(result.data);
      setError(null);
    } else {
      setError(result.error);
    }
  }, []);

  useEffect(() => {
    loadNotesDir();
  }, [loadNotesDir, customDir]);

  const changeDir = async (path: string | null) => {
    const result = await commands.changeNotesDirSetting(path);
    if (result.status === "error") {
      setError(result.error);
      return;
    }
    await refreshSettings();
    await loadNotesDir();
  };

  const handleChoose = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: notesDir || undefined,
    });
    if (typeof selected === "string") {
      await changeDir(selected);
    }
  };

  const handleOpen = async () => {
    const result = await commands.openNotesDir();
    if (result.status === "error") {
      setError(result.error);
    }
  };

  return (
    <SettingContainer
      title={t("settings.notes.folder.title")}
      description={t("settings.notes.folder.description")}
      descriptionMode={descriptionMode}
      grouped={grouped}
      layout="stacked"
    >
      <div className="space-y-2">
        <PathDisplay path={notesDir} onOpen={handleOpen} disabled={!notesDir} />
        <div className="flex gap-2">
          <Button onClick={handleChoose} variant="secondary" size="sm">
            {t("settings.notes.folder.choose")}
          </Button>
          {customDir && (
            <Button
              onClick={() => changeDir(null)}
              variant="secondary"
              size="sm"
            >
              {t("settings.notes.folder.reset")}
            </Button>
          )}
        </div>
        {error && <p className="text-xs text-red-500">{error}</p>}
      </div>
    </SettingContainer>
  );
};
