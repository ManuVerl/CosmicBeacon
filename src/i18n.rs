/// Supported UI languages.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Lang {
    Fr,
    En,
    Es,
    Pt,
    De,
    It,
}

impl Default for Lang {
    fn default() -> Self {
        Self::detect_system()
    }
}

impl Lang {
    /// Attempt to detect the system language; fall back to French.
    pub fn detect_system() -> Self {
        // Try the LANG / LANGUAGE environment variable (Linux/macOS) or
        // the Windows locale string returned by GetUserDefaultLocaleName.
        let locale = std::env::var("LANG")
            .or_else(|_| std::env::var("LANGUAGE"))
            .or_else(|_| std::env::var("LC_ALL"))
            .or_else(|_| std::env::var("LC_MESSAGES"))
            .unwrap_or_default();
        Self::from_locale_str(&locale)
    }

    pub fn from_locale_str(s: &str) -> Self {
        let lower = s.to_lowercase();
        if lower.starts_with("fr") { Self::Fr }
        else if lower.starts_with("es") { Self::Es }
        else if lower.starts_with("pt") { Self::Pt }
        else if lower.starts_with("de") { Self::De }
        else if lower.starts_with("it") { Self::It }
        else if lower.starts_with("en") { Self::En }
        else { Self::Fr }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Fr => "Français",
            Self::En => "English",
            Self::Es => "Español",
            Self::Pt => "Português",
            Self::De => "Deutsch",
            Self::It => "Italiano",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Translation helper macro & all string keys
// ─────────────────────────────────────────────────────────────────────────────

/// Returns the translated string for the given key and language.
pub fn t(lang: &Lang, key: &str) -> &'static str {
    match (lang, key) {
        // ── Menu ─────────────────────────────────────────────────────────────
        (Lang::Fr, "menu.file")             => "Fichier",
        (Lang::En, "menu.file")             => "File",
        (Lang::Es, "menu.file")             => "Archivo",
        (Lang::Pt, "menu.file")             => "Arquivo",
        (Lang::De, "menu.file")             => "Datei",
        (Lang::It, "menu.file")             => "File",

        (Lang::Fr, "menu.refresh")          => "🔄  Rafraîchir",
        (Lang::En, "menu.refresh")          => "🔄  Refresh",
        (Lang::Es, "menu.refresh")          => "🔄  Actualizar",
        (Lang::Pt, "menu.refresh")          => "🔄  Atualizar",
        (Lang::De, "menu.refresh")          => "🔄  Aktualisieren",
        (Lang::It, "menu.refresh")          => "🔄  Aggiorna",

        (Lang::Fr, "menu.quit")             => "Quitter",
        (Lang::En, "menu.quit")             => "Quit",
        (Lang::Es, "menu.quit")             => "Salir",
        (Lang::Pt, "menu.quit")             => "Sair",
        (Lang::De, "menu.quit")             => "Beenden",
        (Lang::It, "menu.quit")             => "Esci",

        (Lang::Fr, "menu.view")             => "Affichage",
        (Lang::En, "menu.view")             => "View",
        (Lang::Es, "menu.view")             => "Vista",
        (Lang::Pt, "menu.view")             => "Vista",
        (Lang::De, "menu.view")             => "Ansicht",
        (Lang::It, "menu.view")             => "Visualizza",

        (Lang::Fr, "menu.fullscreen")       => "⛶  Plein écran",
        (Lang::En, "menu.fullscreen")       => "⛶  Full Screen",
        (Lang::Es, "menu.fullscreen")       => "⛶  Pantalla completa",
        (Lang::Pt, "menu.fullscreen")       => "⛶  Ecrã inteiro",
        (Lang::De, "menu.fullscreen")       => "⛶  Vollbild",
        (Lang::It, "menu.fullscreen")       => "⛶  Schermo intero",

        (Lang::Fr, "menu.windowed")         => "🗗  Mode fenêtre",
        (Lang::En, "menu.windowed")         => "🗗  Windowed",
        (Lang::Es, "menu.windowed")         => "🗗  Modo ventana",
        (Lang::Pt, "menu.windowed")         => "🗗  Modo janela",
        (Lang::De, "menu.windowed")         => "🗗  Fenstermodus",
        (Lang::It, "menu.windowed")         => "🗗  Modalità finestra",

        (Lang::Fr, "menu.edit")             => "Édition",
        (Lang::En, "menu.edit")             => "Edit",
        (Lang::Es, "menu.edit")             => "Edición",
        (Lang::Pt, "menu.edit")             => "Editar",
        (Lang::De, "menu.edit")             => "Bearbeiten",
        (Lang::It, "menu.edit")             => "Modifica",

        (Lang::Fr, "menu.positions")        => "📍  Positions",
        (Lang::En, "menu.positions")        => "📍  Positions",
        (Lang::Es, "menu.positions")        => "📍  Posiciones",
        (Lang::Pt, "menu.positions")        => "📍  Posições",
        (Lang::De, "menu.positions")        => "📍  Positionen",
        (Lang::It, "menu.positions")        => "📍  Posizioni",

        (Lang::Fr, "menu.settings")         => "⚙  Paramètres",
        (Lang::En, "menu.settings")         => "⚙  Settings",
        (Lang::Es, "menu.settings")         => "⚙  Ajustes",
        (Lang::Pt, "menu.settings")         => "⚙  Configurações",
        (Lang::De, "menu.settings")         => "⚙  Einstellungen",
        (Lang::It, "menu.settings")         => "⚙  Impostazioni",

        (Lang::Fr, "menu.help")             => "Aide",
        (Lang::En, "menu.help")             => "Help",
        (Lang::Es, "menu.help")             => "Ayuda",
        (Lang::Pt, "menu.help")             => "Ajuda",
        (Lang::De, "menu.help")             => "Hilfe",
        (Lang::It, "menu.help")             => "Aiuto",

        (Lang::Fr, "menu.about")            => "ℹ  À propos",
        (Lang::En, "menu.about")            => "ℹ  About",
        (Lang::Es, "menu.about")            => "ℹ  Acerca de",
        (Lang::Pt, "menu.about")            => "ℹ  Sobre",
        (Lang::De, "menu.about")            => "ℹ  Über",
        (Lang::It, "menu.about")            => "ℹ  Informazioni",

        (Lang::Fr, "menu.refresh_tooltip")  => "Rafraîchir les événements",
        (Lang::En, "menu.refresh_tooltip")  => "Refresh events",
        (Lang::Es, "menu.refresh_tooltip")  => "Actualizar eventos",
        (Lang::Pt, "menu.refresh_tooltip")  => "Atualizar eventos",
        (Lang::De, "menu.refresh_tooltip")  => "Ereignisse aktualisieren",
        (Lang::It, "menu.refresh_tooltip")  => "Aggiorna eventi",

        (Lang::Fr, "menu.no_position")      => "— aucune position —",
        (Lang::En, "menu.no_position")      => "— no position —",
        (Lang::Es, "menu.no_position")      => "— sin posición —",
        (Lang::Pt, "menu.no_position")      => "— sem posição —",
        (Lang::De, "menu.no_position")      => "— kein Standort —",
        (Lang::It, "menu.no_position")      => "— nessuna posizione —",

        // ── Status bar ───────────────────────────────────────────────────────
        (Lang::Fr, "status.syncing")        => "Synchronisation en cours",
        (Lang::En, "status.syncing")        => "Syncing",
        (Lang::Es, "status.syncing")        => "Sincronizando",
        (Lang::Pt, "status.syncing")        => "Sincronizando",
        (Lang::De, "status.syncing")        => "Synchronisierung",
        (Lang::It, "status.syncing")        => "Sincronizzazione",

        (Lang::Fr, "status.source_s")       => "source(s)",
        (Lang::En, "status.source_s")       => "source(s)",
        (Lang::Es, "status.source_s")       => "fuente(s)",
        (Lang::Pt, "status.source_s")       => "fonte(s)",
        (Lang::De, "status.source_s")       => "Quelle(n)",
        (Lang::It, "status.source_s")       => "sorgente/i",

        (Lang::Fr, "status.error")          => "source(s) en erreur",
        (Lang::En, "status.error")          => "source(s) in error",
        (Lang::Es, "status.error")          => "fuente(s) con error",
        (Lang::Pt, "status.error")          => "fonte(s) com erro",
        (Lang::De, "status.error")          => "Quelle(n) fehlerhaft",
        (Lang::It, "status.error")          => "sorgente/i in errore",

        (Lang::Fr, "status.ok")             => "✅ Synchronisation OK",
        (Lang::En, "status.ok")             => "✅ Sync OK",
        (Lang::Es, "status.ok")             => "✅ Sincronización OK",
        (Lang::Pt, "status.ok")             => "✅ Sincronização OK",
        (Lang::De, "status.ok")             => "✅ Synchronisierung OK",
        (Lang::It, "status.ok")             => "✅ Sincronizzazione OK",

        (Lang::Fr, "status.sync_report")    => "📋 Rapport sync",
        (Lang::En, "status.sync_report")    => "📋 Sync report",
        (Lang::Es, "status.sync_report")    => "📋 Informe sync",
        (Lang::Pt, "status.sync_report")    => "📋 Relatório sync",
        (Lang::De, "status.sync_report")    => "📋 Sync-Bericht",
        (Lang::It, "status.sync_report")    => "📋 Rapporto sync",

        // ── Sidebar filter ───────────────────────────────────────────────────
        (Lang::Fr, "filter.categories")     => "Catégories",
        (Lang::En, "filter.categories")     => "Categories",
        (Lang::Es, "filter.categories")     => "Categorías",
        (Lang::Pt, "filter.categories")     => "Categorias",
        (Lang::De, "filter.categories")     => "Kategorien",
        (Lang::It, "filter.categories")     => "Categorie",

        (Lang::Fr, "filter.all")            => "Tous",
        (Lang::En, "filter.all")            => "All",
        (Lang::Es, "filter.all")            => "Todos",
        (Lang::Pt, "filter.all")            => "Todos",
        (Lang::De, "filter.all")            => "Alle",
        (Lang::It, "filter.all")            => "Tutti",

        (Lang::Fr, "filter.astro")          => "🔭 Astronomie",
        (Lang::En, "filter.astro")          => "🔭 Astronomy",
        (Lang::Es, "filter.astro")          => "🔭 Astronomía",
        (Lang::Pt, "filter.astro")          => "🔭 Astronomia",
        (Lang::De, "filter.astro")          => "🔭 Astronomie",
        (Lang::It, "filter.astro")          => "🔭 Astronomia",

        (Lang::Fr, "filter.radio")          => "📡 Radioastronomie",
        (Lang::En, "filter.radio")          => "📡 Radio Astronomy",
        (Lang::Es, "filter.radio")          => "📡 Radioastronomía",
        (Lang::Pt, "filter.radio")          => "📡 Radioastronomia",
        (Lang::De, "filter.radio")          => "📡 Radioastronomie",
        (Lang::It, "filter.radio")          => "📡 Radioastronomia",

        // ── Event list ───────────────────────────────────────────────────────
        (Lang::Fr, "events.empty")          => "Aucun événement. Appuyez sur 🔄 pour rafraîchir.",
        (Lang::En, "events.empty")          => "No events. Press 🔄 to refresh.",
        (Lang::Es, "events.empty")          => "Sin eventos. Pulse 🔄 para actualizar.",
        (Lang::Pt, "events.empty")          => "Sem eventos. Prima 🔄 para atualizar.",
        (Lang::De, "events.empty")          => "Keine Ereignisse. Drücken Sie 🔄 zum Aktualisieren.",
        (Lang::It, "events.empty")          => "Nessun evento. Premere 🔄 per aggiornare.",

        // ── Temporal navigation bar ───────────────────────────────────────────
        (Lang::Fr, "nav.now")               => "⏱ En ce moment",
        (Lang::En, "nav.now")               => "⏱ Now",
        (Lang::Es, "nav.now")               => "⏱ Ahora",
        (Lang::Pt, "nav.now")               => "⏱ Agora",
        (Lang::De, "nav.now")               => "⏱ Jetzt",
        (Lang::It, "nav.now")               => "⏱ Adesso",

        (Lang::Fr, "nav.goto")              => "Aller à :",
        (Lang::En, "nav.goto")              => "Go to:",
        (Lang::Es, "nav.goto")              => "Ir a:",
        (Lang::Pt, "nav.goto")              => "Ir para:",
        (Lang::De, "nav.goto")              => "Gehe zu:",
        (Lang::It, "nav.goto")              => "Vai a:",

        (Lang::Fr, "nav.date_placeholder")  => "JJ/MM/AAAA HH:MM",
        (Lang::En, "nav.date_placeholder")  => "DD/MM/YYYY HH:MM",
        (Lang::Es, "nav.date_placeholder")  => "DD/MM/AAAA HH:MM",
        (Lang::Pt, "nav.date_placeholder")  => "DD/MM/AAAA HH:MM",
        (Lang::De, "nav.date_placeholder")  => "TT.MM.JJJJ HH:MM",
        (Lang::It, "nav.date_placeholder")  => "GG/MM/AAAA HH:MM",

        // ── Settings window ──────────────────────────────────────────────────
        (Lang::Fr, "settings.title")        => "⚙ Paramètres",
        (Lang::En, "settings.title")        => "⚙ Settings",
        (Lang::Es, "settings.title")        => "⚙ Ajustes",
        (Lang::Pt, "settings.title")        => "⚙ Configurações",
        (Lang::De, "settings.title")        => "⚙ Einstellungen",
        (Lang::It, "settings.title")        => "⚙ Impostazioni",

        (Lang::Fr, "settings.appearance")   => "Apparence",
        (Lang::En, "settings.appearance")   => "Appearance",
        (Lang::Es, "settings.appearance")   => "Apariencia",
        (Lang::Pt, "settings.appearance")   => "Aparência",
        (Lang::De, "settings.appearance")   => "Erscheinungsbild",
        (Lang::It, "settings.appearance")   => "Aspetto",

        (Lang::Fr, "settings.theme")        => "Thème :",
        (Lang::En, "settings.theme")        => "Theme:",
        (Lang::Es, "settings.theme")        => "Tema:",
        (Lang::Pt, "settings.theme")        => "Tema:",
        (Lang::De, "settings.theme")        => "Design:",
        (Lang::It, "settings.theme")        => "Tema:",

        (Lang::Fr, "settings.dark")         => "🌑 Sombre",
        (Lang::En, "settings.dark")         => "🌑 Dark",
        (Lang::Es, "settings.dark")         => "🌑 Oscuro",
        (Lang::Pt, "settings.dark")         => "🌑 Escuro",
        (Lang::De, "settings.dark")         => "🌑 Dunkel",
        (Lang::It, "settings.dark")         => "🌑 Scuro",

        (Lang::Fr, "settings.light")        => "☀ Clair",
        (Lang::En, "settings.light")        => "☀ Light",
        (Lang::Es, "settings.light")        => "☀ Claro",
        (Lang::Pt, "settings.light")        => "☀ Claro",
        (Lang::De, "settings.light")        => "☀ Hell",
        (Lang::It, "settings.light")        => "☀ Chiaro",

        (Lang::Fr, "settings.teal")         => "🟦 Sarcelle",
        (Lang::En, "settings.teal")         => "🟦 Teal",
        (Lang::Es, "settings.teal")         => "🟦 Verde azulado",
        (Lang::Pt, "settings.teal")         => "🟦 Verde-azulado",
        (Lang::De, "settings.teal")         => "🟦 Blaugrün",
        (Lang::It, "settings.teal")         => "🟦 Verde acqua",

        (Lang::Fr, "settings.pink")         => "🩷 Rose",
        (Lang::En, "settings.pink")         => "🩷 Pink",
        (Lang::Es, "settings.pink")         => "🩷 Rosa",
        (Lang::Pt, "settings.pink")         => "🩷 Rosa",
        (Lang::De, "settings.pink")         => "🩷 Pink",
        (Lang::It, "settings.pink")         => "🩷 Rosa",

        (Lang::Fr, "settings.navy")         => "🌌 Marine",
        (Lang::En, "settings.navy")         => "🌌 Navy",
        (Lang::Es, "settings.navy")         => "🌌 Azul marino",
        (Lang::Pt, "settings.navy")         => "🌌 Azul marinho",
        (Lang::De, "settings.navy")         => "🌌 Marineblau",
        (Lang::It, "settings.navy")         => "🌌 Blu navy",

        (Lang::Fr, "settings.update")       => "Mise à jour",
        (Lang::En, "settings.update")       => "Update",
        (Lang::Es, "settings.update")       => "Actualización",
        (Lang::Pt, "settings.update")       => "Atualização",
        (Lang::De, "settings.update")       => "Aktualisierung",
        (Lang::It, "settings.update")       => "Aggiornamento",

        (Lang::Fr, "settings.freq_label")   => "Fréquence de rafraîchissement des événements :",
        (Lang::En, "settings.freq_label")   => "Event refresh frequency:",
        (Lang::Es, "settings.freq_label")   => "Frecuencia de actualización de eventos:",
        (Lang::Pt, "settings.freq_label")   => "Frequência de atualização de eventos:",
        (Lang::De, "settings.freq_label")   => "Häufigkeit der Ereignisaktualisierung:",
        (Lang::It, "settings.freq_label")   => "Frequenza di aggiornamento eventi:",

        (Lang::Fr, "settings.startup")      => "À chaque démarrage",
        (Lang::En, "settings.startup")      => "On every startup",
        (Lang::Es, "settings.startup")      => "En cada inicio",
        (Lang::Pt, "settings.startup")      => "Em cada arranque",
        (Lang::De, "settings.startup")      => "Bei jedem Start",
        (Lang::It, "settings.startup")      => "Ad ogni avvio",

        (Lang::Fr, "settings.weekly")       => "Une fois par semaine",
        (Lang::En, "settings.weekly")       => "Once a week",
        (Lang::Es, "settings.weekly")       => "Una vez por semana",
        (Lang::Pt, "settings.weekly")       => "Uma vez por semana",
        (Lang::De, "settings.weekly")       => "Einmal pro Woche",
        (Lang::It, "settings.weekly")       => "Una volta a settimana",

        (Lang::Fr, "settings.monthly")      => "Une fois par mois",
        (Lang::En, "settings.monthly")      => "Once a month",
        (Lang::Es, "settings.monthly")      => "Una vez al mes",
        (Lang::Pt, "settings.monthly")      => "Uma vez por mês",
        (Lang::De, "settings.monthly")      => "Einmal pro Monat",
        (Lang::It, "settings.monthly")      => "Una volta al mese",

        (Lang::Fr, "settings.language")     => "Langue",
        (Lang::En, "settings.language")     => "Language",
        (Lang::Es, "settings.language")     => "Idioma",
        (Lang::Pt, "settings.language")     => "Idioma",
        (Lang::De, "settings.language")     => "Sprache",
        (Lang::It, "settings.language")     => "Lingua",

        (Lang::Fr, "settings.save")         => "💾 Sauvegarder",
        (Lang::En, "settings.save")         => "💾 Save",
        (Lang::Es, "settings.save")         => "💾 Guardar",
        (Lang::Pt, "settings.save")         => "💾 Guardar",
        (Lang::De, "settings.save")         => "💾 Speichern",
        (Lang::It, "settings.save")         => "💾 Salva",

        (Lang::Fr, "settings.sources")      => "Sources de données",
        (Lang::En, "settings.sources")      => "Data Sources",
        (Lang::Es, "settings.sources")      => "Fuentes de datos",
        (Lang::Pt, "settings.sources")      => "Fontes de dados",
        (Lang::De, "settings.sources")      => "Datenquellen",
        (Lang::It, "settings.sources")      => "Fonti dati",

        (Lang::Fr, "settings.sources_hint") => "Décochez une source pour la désactiver. Son contenu ne sera plus affiché.",
        (Lang::En, "settings.sources_hint") => "Uncheck a source to disable it. Its content will no longer be displayed.",
        (Lang::Es, "settings.sources_hint") => "Desmarque una fuente para desactivarla. Su contenido no se mostrará.",
        (Lang::Pt, "settings.sources_hint") => "Desmarque uma fonte para a desativar. O seu conteúdo não será apresentado.",
        (Lang::De, "settings.sources_hint") => "Deaktivieren Sie eine Quelle per Haken. Ihr Inhalt wird nicht mehr angezeigt.",
        (Lang::It, "settings.sources_hint") => "Deseleziona una fonte per disattivarla. Il suo contenuto non sarà più visualizzato.",

        // ── Positions window ─────────────────────────────────────────────────
        (Lang::Fr, "positions.title")       => "📍 Gestion des positions",
        (Lang::En, "positions.title")       => "📍 Manage positions",
        (Lang::Es, "positions.title")       => "📍 Gestión de posiciones",
        (Lang::Pt, "positions.title")       => "📍 Gestão de posições",
        (Lang::De, "positions.title")       => "📍 Positionen verwalten",
        (Lang::It, "positions.title")       => "📍 Gestione posizioni",

        (Lang::Fr, "positions.delete_tooltip") => "Supprimer",
        (Lang::En, "positions.delete_tooltip") => "Delete",
        (Lang::Es, "positions.delete_tooltip") => "Eliminar",
        (Lang::Pt, "positions.delete_tooltip") => "Apagar",
        (Lang::De, "positions.delete_tooltip") => "Löschen",
        (Lang::It, "positions.delete_tooltip") => "Elimina",

        (Lang::Fr, "positions.add_label")   => "Ajouter une position :",
        (Lang::En, "positions.add_label")   => "Add a position:",
        (Lang::Es, "positions.add_label")   => "Añadir una posición:",
        (Lang::Pt, "positions.add_label")   => "Adicionar uma posição:",
        (Lang::De, "positions.add_label")   => "Standort hinzufügen:",
        (Lang::It, "positions.add_label")   => "Aggiungi una posizione:",

        (Lang::Fr, "positions.name")        => "Nom :",
        (Lang::En, "positions.name")        => "Name:",
        (Lang::Es, "positions.name")        => "Nombre:",
        (Lang::Pt, "positions.name")        => "Nome:",
        (Lang::De, "positions.name")        => "Name:",
        (Lang::It, "positions.name")        => "Nome:",

        (Lang::Fr, "positions.lat")         => "Lat :",
        (Lang::En, "positions.lat")         => "Lat:",
        (Lang::Es, "positions.lat")         => "Lat:",
        (Lang::Pt, "positions.lat")         => "Lat:",
        (Lang::De, "positions.lat")         => "Br.:",
        (Lang::It, "positions.lat")         => "Lat:",

        (Lang::Fr, "positions.lon")         => "Lon :",
        (Lang::En, "positions.lon")         => "Lon:",
        (Lang::Es, "positions.lon")         => "Lon:",
        (Lang::Pt, "positions.lon")         => "Lon:",
        (Lang::De, "positions.lon")         => "Lg.:",
        (Lang::It, "positions.lon")         => "Lon:",

        (Lang::Fr, "positions.add_btn")     => "➕ Ajouter",
        (Lang::En, "positions.add_btn")     => "➕ Add",
        (Lang::Es, "positions.add_btn")     => "➕ Añadir",
        (Lang::Pt, "positions.add_btn")     => "➕ Adicionar",
        (Lang::De, "positions.add_btn")     => "➕ Hinzufügen",
        (Lang::It, "positions.add_btn")     => "➕ Aggiungi",

        (Lang::Fr, "positions.default_name") => "Nouvelle position",
        (Lang::En, "positions.default_name") => "New position",
        (Lang::Es, "positions.default_name") => "Nueva posición",
        (Lang::Pt, "positions.default_name") => "Nova posição",
        (Lang::De, "positions.default_name") => "Neuer Standort",
        (Lang::It, "positions.default_name") => "Nuova posizione",

        (Lang::Fr, "positions.icon_label")  => "Icône :",
        (Lang::En, "positions.icon_label")  => "Icon:",
        (Lang::Es, "positions.icon_label")  => "Icono:",
        (Lang::Pt, "positions.icon_label")  => "Ícone:",
        (Lang::De, "positions.icon_label")  => "Symbol:",
        (Lang::It, "positions.icon_label")  => "Icona:",

        (Lang::Fr, "positions.edit_label")  => "Modifier la position :",
        (Lang::En, "positions.edit_label")  => "Edit position:",
        (Lang::Es, "positions.edit_label")  => "Editar posición:",
        (Lang::Pt, "positions.edit_label")  => "Editar posição:",
        (Lang::De, "positions.edit_label")  => "Standort bearbeiten:",
        (Lang::It, "positions.edit_label")  => "Modifica posizione:",

        (Lang::Fr, "positions.save_btn")    => "💾 Enregistrer",
        (Lang::En, "positions.save_btn")    => "💾 Save",
        (Lang::Es, "positions.save_btn")    => "💾 Guardar",
        (Lang::Pt, "positions.save_btn")    => "💾 Guardar",
        (Lang::De, "positions.save_btn")    => "💾 Speichern",
        (Lang::It, "positions.save_btn")    => "💾 Salva",

        (Lang::Fr, "positions.cancel_btn")  => "Annuler",
        (Lang::En, "positions.cancel_btn")  => "Cancel",
        (Lang::Es, "positions.cancel_btn")  => "Cancelar",
        (Lang::Pt, "positions.cancel_btn")  => "Cancelar",
        (Lang::De, "positions.cancel_btn")  => "Abbrechen",
        (Lang::It, "positions.cancel_btn")  => "Annulla",

        (Lang::Fr, "positions.geolocate_tooltip") => "Détecter automatiquement les coordonnées",
        (Lang::En, "positions.geolocate_tooltip") => "Auto-detect coordinates",
        (Lang::Es, "positions.geolocate_tooltip") => "Detectar coordenadas automáticamente",
        (Lang::Pt, "positions.geolocate_tooltip") => "Detectar coordenadas automaticamente",
        (Lang::De, "positions.geolocate_tooltip") => "Koordinaten automatisch erkennen",
        (Lang::It, "positions.geolocate_tooltip") => "Rileva coordinate automaticamente",

        (Lang::Fr, "positions.geo_searching") => "Recherche en cours…",
        (Lang::En, "positions.geo_searching") => "Searching…",
        (Lang::Es, "positions.geo_searching") => "Buscando…",
        (Lang::Pt, "positions.geo_searching") => "A procurar…",
        (Lang::De, "positions.geo_searching") => "Suche läuft…",
        (Lang::It, "positions.geo_searching") => "Ricerca in corso…",

        (Lang::Fr, "positions.geo_err_no_result")  => "⚠ Aucun résultat trouvé pour ce nom.",
        (Lang::En, "positions.geo_err_no_result")  => "⚠ No result found for this name.",
        (Lang::Es, "positions.geo_err_no_result")  => "⚠ No se encontró resultado para este nombre.",
        (Lang::Pt, "positions.geo_err_no_result")  => "⚠ Nenhum resultado encontrado para este nome.",
        (Lang::De, "positions.geo_err_no_result")  => "⚠ Kein Ergebnis für diesen Namen gefunden.",
        (Lang::It, "positions.geo_err_no_result")  => "⚠ Nessun risultato trovato per questo nome.",

        (Lang::Fr, "positions.geo_err_network")    => "⚠ Erreur réseau lors de la recherche.",
        (Lang::En, "positions.geo_err_network")    => "⚠ Network error during search.",
        (Lang::Es, "positions.geo_err_network")    => "⚠ Error de red durante la búsqueda.",
        (Lang::Pt, "positions.geo_err_network")    => "⚠ Erro de rede durante a pesquisa.",
        (Lang::De, "positions.geo_err_network")    => "⚠ Netzwerkfehler bei der Suche.",
        (Lang::It, "positions.geo_err_network")    => "⚠ Errore di rete durante la ricerca.",

        (Lang::Fr, "positions.geo_err_no_location") => "⚠ Impossible de détecter la position (GPS non disponible, géolocalisation IP échouée).",
        (Lang::En, "positions.geo_err_no_location") => "⚠ Could not detect position (GPS unavailable, IP geolocation failed).",
        (Lang::Es, "positions.geo_err_no_location") => "⚠ No se pudo detectar la posición (GPS no disponible, geolocalización IP fallida).",
        (Lang::Pt, "positions.geo_err_no_location") => "⚠ Não foi possível detectar a posição (GPS indisponível, geolocalização IP falhou).",
        (Lang::De, "positions.geo_err_no_location") => "⚠ Position konnte nicht erkannt werden (GPS nicht verfügbar, IP-Geolokalisierung fehlgeschlagen).",
        (Lang::It, "positions.geo_err_no_location") => "⚠ Impossibile rilevare la posizione (GPS non disponibile, geolocalizzazione IP fallita).",

        // ── Sync report window ───────────────────────────────────────────────
        (Lang::Fr, "sync.title")            => "📋 Rapport de synchronisation",
        (Lang::En, "sync.title")            => "📋 Sync Report",
        (Lang::Es, "sync.title")            => "📋 Informe de sincronización",
        (Lang::Pt, "sync.title")            => "📋 Relatório de sincronização",
        (Lang::De, "sync.title")            => "📋 Synchronisierungsbericht",
        (Lang::It, "sync.title")            => "📋 Rapporto di sincronizzazione",

        (Lang::Fr, "sync.none")             => "Aucune synchronisation effectuée.",
        (Lang::En, "sync.none")             => "No synchronisation performed.",
        (Lang::Es, "sync.none")             => "Ninguna sincronización realizada.",
        (Lang::Pt, "sync.none")             => "Nenhuma sincronização efetuada.",
        (Lang::De, "sync.none")             => "Keine Synchronisierung durchgeführt.",
        (Lang::It, "sync.none")             => "Nessuna sincronizzazione eseguita.",

        (Lang::Fr, "sync.col_source")       => "Source",
        (Lang::En, "sync.col_source")       => "Source",
        (Lang::Es, "sync.col_source")       => "Fuente",
        (Lang::Pt, "sync.col_source")       => "Fonte",
        (Lang::De, "sync.col_source")       => "Quelle",
        (Lang::It, "sync.col_source")       => "Sorgente",

        (Lang::Fr, "sync.col_status")       => "Statut",
        (Lang::En, "sync.col_status")       => "Status",
        (Lang::Es, "sync.col_status")       => "Estado",
        (Lang::Pt, "sync.col_status")       => "Estado",
        (Lang::De, "sync.col_status")       => "Status",
        (Lang::It, "sync.col_status")       => "Stato",

        (Lang::Fr, "sync.col_last")         => "Dernière sync",
        (Lang::En, "sync.col_last")         => "Last sync",
        (Lang::Es, "sync.col_last")         => "Última sync",
        (Lang::Pt, "sync.col_last")         => "Última sync",
        (Lang::De, "sync.col_last")         => "Letzte Sync",
        (Lang::It, "sync.col_last")         => "Ultima sync",

        (Lang::Fr, "sync.ok")               => "✅ OK",
        (Lang::En, "sync.ok")               => "✅ OK",
        (Lang::Es, "sync.ok")               => "✅ OK",
        (Lang::Pt, "sync.ok")               => "✅ OK",
        (Lang::De, "sync.ok")               => "✅ OK",
        (Lang::It, "sync.ok")               => "✅ OK",

        (Lang::Fr, "sync.error")            => "❌ Erreur",
        (Lang::En, "sync.error")            => "❌ Error",
        (Lang::Es, "sync.error")            => "❌ Error",
        (Lang::Pt, "sync.error")            => "❌ Erro",
        (Lang::De, "sync.error")            => "❌ Fehler",
        (Lang::It, "sync.error")            => "❌ Errore",

        (Lang::Fr, "sync.pending")          => "En cours…",
        (Lang::En, "sync.pending")          => "In progress…",
        (Lang::Es, "sync.pending")          => "En curso…",
        (Lang::Pt, "sync.pending")          => "Em curso…",
        (Lang::De, "sync.pending")          => "Läuft…",
        (Lang::It, "sync.pending")          => "In corso…",

        (Lang::Fr, "sync.disabled")         => "⏸ Désactivé",
        (Lang::En, "sync.disabled")         => "⏸ Disabled",
        (Lang::Es, "sync.disabled")         => "⏸ Desactivado",
        (Lang::Pt, "sync.disabled")         => "⏸ Desativado",
        (Lang::De, "sync.disabled")         => "⏸ Deaktiviert",
        (Lang::It, "sync.disabled")         => "⏸ Disabilitato",

        (Lang::Fr, "sync.refresh_now")      => "🔄 Rafraîchir maintenant",
        (Lang::En, "sync.refresh_now")      => "🔄 Refresh now",
        (Lang::Es, "sync.refresh_now")      => "🔄 Actualizar ahora",
        (Lang::Pt, "sync.refresh_now")      => "🔄 Atualizar agora",
        (Lang::De, "sync.refresh_now")      => "🔄 Jetzt aktualisieren",
        (Lang::It, "sync.refresh_now")      => "🔄 Aggiorna ora",

        // ── About window ─────────────────────────────────────────────────────
        (Lang::Fr, "about.title")           => "ℹ  À propos de Cosmic Beacon",
        (Lang::En, "about.title")           => "ℹ  About Cosmic Beacon",
        (Lang::Es, "about.title")           => "ℹ  Acerca de Cosmic Beacon",
        (Lang::Pt, "about.title")           => "ℹ  Sobre Cosmic Beacon",
        (Lang::De, "about.title")           => "ℹ  Über Cosmic Beacon",
        (Lang::It, "about.title")           => "ℹ  Informazioni su Cosmic Beacon",

        (Lang::Fr, "about.subtitle")        => "Traqueur d'événements astronomiques & radioastronomiques",
        (Lang::En, "about.subtitle")        => "Astronomical & radio-astronomical event tracker",
        (Lang::Es, "about.subtitle")        => "Rastreador de eventos astronómicos y radioastronómicos",
        (Lang::Pt, "about.subtitle")        => "Rastreador de eventos astronómicos e radioastronómicos",
        (Lang::De, "about.subtitle")        => "Tracker für astronomische & radioastronomische Ereignisse",
        (Lang::It, "about.subtitle")        => "Tracciatore di eventi astronomici e radioastronomici",

        (Lang::Fr, "about.authors_heading") => "Auteurs & Développement",
        (Lang::En, "about.authors_heading") => "Authors & Development",
        (Lang::Es, "about.authors_heading") => "Autores y Desarrollo",
        (Lang::Pt, "about.authors_heading") => "Autores e Desenvolvimento",
        (Lang::De, "about.authors_heading") => "Autoren & Entwicklung",
        (Lang::It, "about.authors_heading") => "Autori e Sviluppo",

        (Lang::Fr, "about.developed_by")    => "Conçu par",
        (Lang::En, "about.developed_by")    => "Designed by",
        (Lang::Es, "about.developed_by")    => "Diseñado por",
        (Lang::Pt, "about.developed_by")    => "Concebido por",
        (Lang::De, "about.developed_by")    => "Entworfen von",
        (Lang::It, "about.developed_by")    => "Progettato da",

        (Lang::Fr, "about.with_help")       => ", avec l'aide de",
        (Lang::En, "about.with_help")       => ", with the help of",
        (Lang::Es, "about.with_help")       => ", con la ayuda de",
        (Lang::Pt, "about.with_help")       => ", com a ajuda de",
        (Lang::De, "about.with_help")       => ", mit Unterstützung von",
        (Lang::It, "about.with_help")       => ", con il contributo di",

        (Lang::Fr, "about.license_heading") => "Licence",
        (Lang::En, "about.license_heading") => "License",
        (Lang::Es, "about.license_heading") => "Licencia",
        (Lang::Pt, "about.license_heading") => "Licença",
        (Lang::De, "about.license_heading") => "Lizenz",
        (Lang::It, "about.license_heading") => "Licenza",

        (Lang::Fr, "about.license_text")    => "Distribué sous les termes de la",
        (Lang::En, "about.license_text")    => "Distributed under the terms of the",
        (Lang::Es, "about.license_text")    => "Distribuido bajo los términos de la",
        (Lang::Pt, "about.license_text")    => "Distribuído nos termos da",
        (Lang::De, "about.license_text")    => "Verteilt unter den Bedingungen der",
        (Lang::It, "about.license_text")    => "Distribuito sotto i termini della",

        (Lang::Fr, "about.license_name")    => "Licence MIT (Open Source)",
        (Lang::En, "about.license_name")    => "MIT License (Open Source)",
        (Lang::Es, "about.license_name")    => "Licencia MIT (Open Source)",
        (Lang::Pt, "about.license_name")    => "Licença MIT (Open Source)",
        (Lang::De, "about.license_name")    => "MIT-Lizenz (Open Source)",
        (Lang::It, "about.license_name")    => "Licenza MIT (Open Source)",

        (Lang::Fr, "about.sources_heading") => "Sources de données & Références",
        (Lang::En, "about.sources_heading") => "Data Sources & References",
        (Lang::Es, "about.sources_heading") => "Fuentes de datos y Referencias",
        (Lang::Pt, "about.sources_heading") => "Fontes de dados e Referências",
        (Lang::De, "about.sources_heading") => "Datenquellen & Referenzen",
        (Lang::It, "about.sources_heading") => "Fonti dati e Riferimenti",

        (Lang::Fr, "about.sources_desc")    => "Cosmic Beacon s'appuie sur les sources et catalogues ouverts suivants :",
        (Lang::En, "about.sources_desc")    => "Cosmic Beacon relies on the following open sources and catalogues:",
        (Lang::Es, "about.sources_desc")    => "Cosmic Beacon se apoya en las siguientes fuentes y catálogos abiertos:",
        (Lang::Pt, "about.sources_desc")    => "Cosmic Beacon baseia-se nas seguintes fontes e catálogos abertos:",
        (Lang::De, "about.sources_desc")    => "Cosmic Beacon stützt sich auf folgende offene Quellen und Kataloge:",
        (Lang::It, "about.sources_desc")    => "Cosmic Beacon si basa sulle seguenti fonti e cataloghi aperti:",

        (Lang::Fr, "about.src_iss")         => "Passages ISS (TLE) :",
        (Lang::En, "about.src_iss")         => "ISS Passes (TLE):",
        (Lang::Es, "about.src_iss")         => "Pasos ISS (TLE):",
        (Lang::Pt, "about.src_iss")         => "Passagens ISS (TLE):",
        (Lang::De, "about.src_iss")         => "ISS-Überflüge (TLE):",
        (Lang::It, "about.src_iss")         => "Passaggi ISS (TLE):",

        (Lang::Fr, "about.src_planets")     => "Éphémérides planétaires :",
        (Lang::En, "about.src_planets")     => "Planetary ephemerides:",
        (Lang::Es, "about.src_planets")     => "Efemérides planetarias:",
        (Lang::Pt, "about.src_planets")     => "Efemérides planetárias:",
        (Lang::De, "about.src_planets")     => "Planetenephemeriden:",
        (Lang::It, "about.src_planets")     => "Effemeridi planetarie:",

        (Lang::Fr, "about.src_meteors")     => "Essaims de météores :",
        (Lang::En, "about.src_meteors")     => "Meteor showers:",
        (Lang::Es, "about.src_meteors")     => "Lluvias de meteoros:",
        (Lang::Pt, "about.src_meteors")     => "Chuvas de meteoros:",
        (Lang::De, "about.src_meteors")     => "Meteorschauer:",
        (Lang::It, "about.src_meteors")     => "Sciami meteorici:",

        (Lang::Fr, "about.src_comets")      => "Comètes observables :",
        (Lang::En, "about.src_comets")      => "Observable comets:",
        (Lang::Es, "about.src_comets")      => "Cometas observables:",
        (Lang::Pt, "about.src_comets")      => "Cometas observáveis:",
        (Lang::De, "about.src_comets")      => "Beobachtbare Kometen:",
        (Lang::It, "about.src_comets")      => "Comete osservabili:",

        (Lang::Fr, "about.src_iss_radio")   => "Contacts radioamateurs ISS :",
        (Lang::En, "about.src_iss_radio")   => "ISS amateur radio contacts:",
        (Lang::Es, "about.src_iss_radio")   => "Contactos radioaficionados ISS:",
        (Lang::Pt, "about.src_iss_radio")   => "Contactos radioamador ISS:",
        (Lang::De, "about.src_iss_radio")   => "ISS Amateurfunk-Kontakte:",
        (Lang::It, "about.src_iss_radio")   => "Contatti radioamatoriali ISS:",

        (Lang::Fr, "about.src_solar")       => "Transit solaire & éphémérides :",
        (Lang::En, "about.src_solar")       => "Solar transit & ephemerides:",
        (Lang::Es, "about.src_solar")       => "Tránsito solar y efemérides:",
        (Lang::Pt, "about.src_solar")       => "Trânsito solar e efemérides:",
        (Lang::De, "about.src_solar")       => "Sonnentransit & Ephemeridem:",
        (Lang::It, "about.src_solar")       => "Transito solare ed effemeridi:",

        (Lang::Fr, "about.src_solar_noaa")  => "Algorithmes solaires NOAA",
        (Lang::En, "about.src_solar_noaa")  => "NOAA Solar Algorithms",
        (Lang::Es, "about.src_solar_noaa")  => "Algoritmos solares NOAA",
        (Lang::Pt, "about.src_solar_noaa")  => "Algoritmos solares NOAA",
        (Lang::De, "about.src_solar_noaa")  => "NOAA-Sonnenalgorithmen",
        (Lang::It, "about.src_solar_noaa")  => "Algoritmi solari NOAA",

        (Lang::Fr, "about.src_milky_way")   => "Transit Voie Lactée (centre galactique) :",
        (Lang::En, "about.src_milky_way")   => "Milky Way transit (galactic centre):",
        (Lang::Es, "about.src_milky_way")   => "Tránsito Vía Láctea (centro galáctico):",
        (Lang::Pt, "about.src_milky_way")   => "Trânsito Via Láctea (centro galáctico):",
        (Lang::De, "about.src_milky_way")   => "Milchstraßentransit (galaktisches Zentrum):",
        (Lang::It, "about.src_milky_way")   => "Transito Via Lattea (centro galattico):",

        (Lang::Fr, "about.close")           => "Fermer",
        (Lang::En, "about.close")           => "Close",
        (Lang::Es, "about.close")           => "Cerrar",
        (Lang::Pt, "about.close")           => "Fechar",
        (Lang::De, "about.close")           => "Schließen",
        (Lang::It, "about.close")           => "Chiudi",

        // ── Sidebar info ─────────────────────────────────────────────────────
        (Lang::Fr, "sidebar.no_position")   => "📍 Aucune position définie",
        (Lang::En, "sidebar.no_position")   => "📍 No position defined",
        (Lang::Es, "sidebar.no_position")   => "📍 Sin posición definida",
        (Lang::Pt, "sidebar.no_position")   => "📍 Sem posição definida",
        (Lang::De, "sidebar.no_position")   => "📍 Kein Standort definiert",
        (Lang::It, "sidebar.no_position")   => "📍 Nessuna posizione definita",

        (Lang::Fr, "sidebar.elevation")     => "Élévation : ",
        (Lang::En, "sidebar.elevation")     => "Elevation: ",
        (Lang::Es, "sidebar.elevation")     => "Elevación: ",
        (Lang::Pt, "sidebar.elevation")     => "Elevação: ",
        (Lang::De, "sidebar.elevation")     => "Elevation: ",
        (Lang::It, "sidebar.elevation")     => "Elevazione: ",

        // ── Sun widget labels ─────────────────────────────────────────────────
        (Lang::Fr, "sun.daylight")          => "Durée du jour",
        (Lang::En, "sun.daylight")          => "Daylight",
        (Lang::Es, "sun.daylight")          => "Duración del día",
        (Lang::Pt, "sun.daylight")          => "Duração do dia",
        (Lang::De, "sun.daylight")          => "Tageslicht",
        (Lang::It, "sun.daylight")          => "Luce del giorno",

        (Lang::Fr, "sun.rise")              => "Lever",
        (Lang::En, "sun.rise")              => "Rise",
        (Lang::Es, "sun.rise")              => "Salida",
        (Lang::Pt, "sun.rise")              => "Nascente",
        (Lang::De, "sun.rise")              => "Aufgang",
        (Lang::It, "sun.rise")              => "Alba",

        (Lang::Fr, "sun.set")               => "Coucher",
        (Lang::En, "sun.set")               => "Set",
        (Lang::Es, "sun.set")               => "Puesta",
        (Lang::Pt, "sun.set")               => "Poente",
        (Lang::De, "sun.set")               => "Untergang",
        (Lang::It, "sun.set")               => "Tramonto",

        (Lang::Fr, "sun.altitude")          => "Altitude",
        (Lang::En, "sun.altitude")          => "Altitude",
        (Lang::Es, "sun.altitude")          => "Altitud",
        (Lang::Pt, "sun.altitude")          => "Altitude",
        (Lang::De, "sun.altitude")          => "Höhe",
        (Lang::It, "sun.altitude")          => "Altitudine",

        (Lang::Fr, "sun.direction")         => "Direction",
        (Lang::En, "sun.direction")         => "Direction",
        (Lang::Es, "sun.direction")         => "Dirección",
        (Lang::Pt, "sun.direction")         => "Direção",
        (Lang::De, "sun.direction")         => "Richtung",
        (Lang::It, "sun.direction")         => "Direzione",

        (Lang::Fr, "sun.no_rise")           => "Ne se lève pas",
        (Lang::En, "sun.no_rise")           => "Does not rise",
        (Lang::Es, "sun.no_rise")           => "No sale",
        (Lang::Pt, "sun.no_rise")           => "Não nasce",
        (Lang::De, "sun.no_rise")           => "Geht nicht auf",
        (Lang::It, "sun.no_rise")           => "Non sorge",

        (Lang::Fr, "sun.hour")              => "h",
        (Lang::En, "sun.hour")              => "h",
        (Lang::Es, "sun.hour")              => "h",
        (Lang::Pt, "sun.hour")              => "h",
        (Lang::De, "sun.hour")              => "Std.",
        (Lang::It, "sun.hour")              => "h",

        (Lang::Fr, "sun.minute")            => "min",
        (Lang::En, "sun.minute")            => "min",
        (Lang::Es, "sun.minute")            => "min",
        (Lang::Pt, "sun.minute")            => "min",
        (Lang::De, "sun.minute")            => "Min.",
        (Lang::It, "sun.minute")            => "min",

        (Lang::Fr, "sidebar.illum")         => "éclairée",
        (Lang::En, "sidebar.illum")         => "illuminated",
        (Lang::Es, "sidebar.illum")         => "iluminada",
        (Lang::Pt, "sidebar.illum")         => "iluminada",
        (Lang::De, "sidebar.illum")         => "beleuchtet",
        (Lang::It, "sidebar.illum")         => "illuminata",

        (Lang::Fr, "sidebar.lon_card_w")    => "O",
        (Lang::En, "sidebar.lon_card_w")    => "W",
        (Lang::Es, "sidebar.lon_card_w")    => "O",
        (Lang::Pt, "sidebar.lon_card_w")    => "O",
        (Lang::De, "sidebar.lon_card_w")    => "W",
        (Lang::It, "sidebar.lon_card_w")    => "O",

        // ── Moon widget labels ────────────────────────────────────────────────
        (Lang::Fr, "moon.illumination")     => "Illumination",
        (Lang::En, "moon.illumination")     => "Illumination",
        (Lang::Es, "moon.illumination")     => "Iluminación",
        (Lang::Pt, "moon.illumination")     => "Iluminação",
        (Lang::De, "moon.illumination")     => "Beleuchtung",
        (Lang::It, "moon.illumination")     => "Illuminazione",

        (Lang::Fr, "moon.rise")             => "Lever",
        (Lang::En, "moon.rise")             => "Rise",
        (Lang::Es, "moon.rise")             => "Salida",
        (Lang::Pt, "moon.rise")             => "Nascente",
        (Lang::De, "moon.rise")             => "Aufgang",
        (Lang::It, "moon.rise")             => "Sorge",

        (Lang::Fr, "moon.set")              => "Coucher",
        (Lang::En, "moon.set")              => "Set",
        (Lang::Es, "moon.set")              => "Puesta",
        (Lang::Pt, "moon.set")              => "Poente",
        (Lang::De, "moon.set")              => "Untergang",
        (Lang::It, "moon.set")              => "Tramonta",

        (Lang::Fr, "moon.altitude")         => "Altitude",
        (Lang::En, "moon.altitude")         => "Altitude",
        (Lang::Es, "moon.altitude")         => "Altitud",
        (Lang::Pt, "moon.altitude")         => "Altitude",
        (Lang::De, "moon.altitude")         => "Höhe",
        (Lang::It, "moon.altitude")         => "Altitudine",

        (Lang::Fr, "moon.above_horizon")    => "au-dessus de l'horizon",
        (Lang::En, "moon.above_horizon")    => "above horizon",
        (Lang::Es, "moon.above_horizon")    => "sobre el horizonte",
        (Lang::Pt, "moon.above_horizon")    => "acima do horizonte",
        (Lang::De, "moon.above_horizon")    => "über dem Horizont",
        (Lang::It, "moon.above_horizon")    => "sopra l'orizzonte",

        (Lang::Fr, "moon.below_horizon")    => "sous l'horizon",
        (Lang::En, "moon.below_horizon")    => "below horizon",
        (Lang::Es, "moon.below_horizon")    => "bajo el horizonte",
        (Lang::Pt, "moon.below_horizon")    => "abaixo do horizonte",
        (Lang::De, "moon.below_horizon")    => "unter dem Horizont",
        (Lang::It, "moon.below_horizon")    => "sotto l'orizzonte",

        (Lang::Fr, "moon.direction")        => "Direction",
        (Lang::En, "moon.direction")        => "Direction",
        (Lang::Es, "moon.direction")        => "Dirección",
        (Lang::Pt, "moon.direction")        => "Direção",
        (Lang::De, "moon.direction")        => "Richtung",
        (Lang::It, "moon.direction")        => "Direzione",

        (Lang::Fr, "moon.no_rise")          => "Ne se lève pas",
        (Lang::En, "moon.no_rise")          => "Does not rise",
        (Lang::Es, "moon.no_rise")          => "No sale",
        (Lang::Pt, "moon.no_rise")          => "Não nasce",
        (Lang::De, "moon.no_rise")          => "Geht nicht auf",
        (Lang::It, "moon.no_rise")          => "Non sorge",

        // ── 16-point compass ─────────────────────────────────────────────────
        // These are universal abbreviations — most languages keep them in Latin.
        // French/Spanish/Portuguese/Italian use N/S/E but O (Ouest/Oeste) for West.
        (_, "compass.N")   => "N",
        (_, "compass.NNE") => "NNE",
        (_, "compass.NE")  => "NE",
        (_, "compass.ENE") => "ENE",
        (_, "compass.E")   => "E",
        (_, "compass.ESE") => "ESE",
        (_, "compass.SE")  => "SE",
        (_, "compass.SSE") => "SSE",
        (_, "compass.S")   => "S",
        (_, "compass.SSW") => match lang {
            Lang::Fr | Lang::Es | Lang::Pt | Lang::It => "SSO",
            _ => "SSW",
        },
        (_, "compass.SW")  => match lang {
            Lang::Fr | Lang::Es | Lang::Pt | Lang::It => "SO",
            _ => "SW",
        },
        (_, "compass.WSW") => match lang {
            Lang::Fr | Lang::Es | Lang::Pt | Lang::It => "OSO",
            _ => "WSW",
        },
        (_, "compass.W")   => match lang {
            Lang::Fr | Lang::Es | Lang::Pt | Lang::It => "O",
            _ => "W",
        },
        (_, "compass.WNW") => match lang {
            Lang::Fr | Lang::Es | Lang::Pt | Lang::It => "ONO",
            _ => "WNW",
        },
        (_, "compass.NW")  => match lang {
            Lang::Fr | Lang::Es | Lang::Pt | Lang::It => "NO",
            _ => "NW",
        },
        (_, "compass.NNW") => match lang {
            Lang::Fr | Lang::Es | Lang::Pt | Lang::It => "NNO",
            _ => "NNW",
        },

        // ── Moon phase names ─────────────────────────────────────────────────
        (Lang::Fr, "moon.new")              => "Nouvelle Lune",
        (Lang::En, "moon.new")              => "New Moon",
        (Lang::Es, "moon.new")              => "Luna Nueva",
        (Lang::Pt, "moon.new")              => "Lua Nova",
        (Lang::De, "moon.new")              => "Neumond",
        (Lang::It, "moon.new")              => "Luna Nuova",

        (Lang::Fr, "moon.waxing_crescent")  => "Premier Croissant",
        (Lang::En, "moon.waxing_crescent")  => "Waxing Crescent",
        (Lang::Es, "moon.waxing_crescent")  => "Cuarto Creciente",
        (Lang::Pt, "moon.waxing_crescent")  => "Lua Crescente",
        (Lang::De, "moon.waxing_crescent")  => "Zunehmend",
        (Lang::It, "moon.waxing_crescent")  => "Luna Crescente",

        (Lang::Fr, "moon.first_quarter")    => "Premier Quartier",
        (Lang::En, "moon.first_quarter")    => "First Quarter",
        (Lang::Es, "moon.first_quarter")    => "Primer Cuarto",
        (Lang::Pt, "moon.first_quarter")    => "Quarto Crescente",
        (Lang::De, "moon.first_quarter")    => "Erstes Viertel",
        (Lang::It, "moon.first_quarter")    => "Primo Quarto",

        (Lang::Fr, "moon.waxing_gibbous")   => "Gibbeuse Croissante",
        (Lang::En, "moon.waxing_gibbous")   => "Waxing Gibbous",
        (Lang::Es, "moon.waxing_gibbous")   => "Gibosa Creciente",
        (Lang::Pt, "moon.waxing_gibbous")   => "Gibosa Crescente",
        (Lang::De, "moon.waxing_gibbous")   => "Zunehmend Gibbös",
        (Lang::It, "moon.waxing_gibbous")   => "Gibbosa Crescente",

        (Lang::Fr, "moon.full")             => "Pleine Lune",
        (Lang::En, "moon.full")             => "Full Moon",
        (Lang::Es, "moon.full")             => "Luna Llena",
        (Lang::Pt, "moon.full")             => "Lua Cheia",
        (Lang::De, "moon.full")             => "Vollmond",
        (Lang::It, "moon.full")             => "Luna Piena",

        (Lang::Fr, "moon.waning_gibbous")   => "Gibbeuse Décroissante",
        (Lang::En, "moon.waning_gibbous")   => "Waning Gibbous",
        (Lang::Es, "moon.waning_gibbous")   => "Gibosa Menguante",
        (Lang::Pt, "moon.waning_gibbous")   => "Gibosa Minguante",
        (Lang::De, "moon.waning_gibbous")   => "Abnehmend Gibbös",
        (Lang::It, "moon.waning_gibbous")   => "Gibbosa Calante",

        (Lang::Fr, "moon.last_quarter")     => "Dernier Quartier",
        (Lang::En, "moon.last_quarter")     => "Last Quarter",
        (Lang::Es, "moon.last_quarter")     => "Último Cuarto",
        (Lang::Pt, "moon.last_quarter")     => "Último Quarto",
        (Lang::De, "moon.last_quarter")     => "Letztes Viertel",
        (Lang::It, "moon.last_quarter")     => "Ultimo Quarto",

        (Lang::Fr, "moon.waning_crescent")  => "Dernier Croissant",
        (Lang::En, "moon.waning_crescent")  => "Waning Crescent",
        (Lang::Es, "moon.waning_crescent")  => "Menguante",
        (Lang::Pt, "moon.waning_crescent")  => "Lua Minguante",
        (Lang::De, "moon.waning_crescent")  => "Abnehmend",
        (Lang::It, "moon.waning_crescent")  => "Luna Calante",

        // ── Weekdays ─────────────────────────────────────────────────────────
        (_, "day.mon") => match lang {
            Lang::Fr => "Lundi",   Lang::En => "Monday",    Lang::Es => "Lunes",
            Lang::Pt => "Segunda", Lang::De => "Montag",    Lang::It => "Lunedì",
        },
        (_, "day.tue") => match lang {
            Lang::Fr => "Mardi",   Lang::En => "Tuesday",   Lang::Es => "Martes",
            Lang::Pt => "Terça",   Lang::De => "Dienstag",  Lang::It => "Martedì",
        },
        (_, "day.wed") => match lang {
            Lang::Fr => "Mercredi", Lang::En => "Wednesday", Lang::Es => "Miércoles",
            Lang::Pt => "Quarta",   Lang::De => "Mittwoch",  Lang::It => "Mercoledì",
        },
        (_, "day.thu") => match lang {
            Lang::Fr => "Jeudi",   Lang::En => "Thursday",  Lang::Es => "Jueves",
            Lang::Pt => "Quinta",  Lang::De => "Donnerstag", Lang::It => "Giovedì",
        },
        (_, "day.fri") => match lang {
            Lang::Fr => "Vendredi", Lang::En => "Friday",   Lang::Es => "Viernes",
            Lang::Pt => "Sexta",    Lang::De => "Freitag",  Lang::It => "Venerdì",
        },
        (_, "day.sat") => match lang {
            Lang::Fr => "Samedi",  Lang::En => "Saturday",  Lang::Es => "Sábado",
            Lang::Pt => "Sábado",  Lang::De => "Samstag",   Lang::It => "Sabato",
        },
        (_, "day.sun") => match lang {
            Lang::Fr => "Dimanche", Lang::En => "Sunday",   Lang::Es => "Domingo",
            Lang::Pt => "Domingo",  Lang::De => "Sonntag",  Lang::It => "Domenica",
        },

        // ── Months ───────────────────────────────────────────────────────────
        (_, "month.1") => match lang {
            Lang::Fr => "janvier",    Lang::En => "January",   Lang::Es => "enero",
            Lang::Pt => "janeiro",    Lang::De => "Januar",    Lang::It => "gennaio",
        },
        (_, "month.2") => match lang {
            Lang::Fr => "février",    Lang::En => "February",  Lang::Es => "febrero",
            Lang::Pt => "fevereiro",  Lang::De => "Februar",   Lang::It => "febbraio",
        },
        (_, "month.3") => match lang {
            Lang::Fr => "mars",       Lang::En => "March",     Lang::Es => "marzo",
            Lang::Pt => "março",      Lang::De => "März",      Lang::It => "marzo",
        },
        (_, "month.4") => match lang {
            Lang::Fr => "avril",      Lang::En => "April",     Lang::Es => "abril",
            Lang::Pt => "abril",      Lang::De => "April",     Lang::It => "aprile",
        },
        (_, "month.5") => match lang {
            Lang::Fr => "mai",        Lang::En => "May",       Lang::Es => "mayo",
            Lang::Pt => "maio",       Lang::De => "Mai",       Lang::It => "maggio",
        },
        (_, "month.6") => match lang {
            Lang::Fr => "juin",       Lang::En => "June",      Lang::Es => "junio",
            Lang::Pt => "junho",      Lang::De => "Juni",      Lang::It => "giugno",
        },
        (_, "month.7") => match lang {
            Lang::Fr => "juillet",    Lang::En => "July",      Lang::Es => "julio",
            Lang::Pt => "julho",      Lang::De => "Juli",      Lang::It => "luglio",
        },
        (_, "month.8") => match lang {
            Lang::Fr => "août",       Lang::En => "August",    Lang::Es => "agosto",
            Lang::Pt => "agosto",     Lang::De => "August",    Lang::It => "agosto",
        },
        (_, "month.9") => match lang {
            Lang::Fr => "septembre",  Lang::En => "September", Lang::Es => "septiembre",
            Lang::Pt => "setembro",   Lang::De => "September", Lang::It => "settembre",
        },
        (_, "month.10") => match lang {
            Lang::Fr => "octobre",    Lang::En => "October",   Lang::Es => "octubre",
            Lang::Pt => "outubro",    Lang::De => "Oktober",   Lang::It => "ottobre",
        },
        (_, "month.11") => match lang {
            Lang::Fr => "novembre",   Lang::En => "November",  Lang::Es => "noviembre",
            Lang::Pt => "novembro",   Lang::De => "November",  Lang::It => "novembre",
        },
        (_, "month.12") => match lang {
            Lang::Fr => "décembre",   Lang::En => "December",  Lang::Es => "diciembre",
            Lang::Pt => "dezembro",   Lang::De => "Dezember",  Lang::It => "dicembre",
        },

        // ── Fallback ─────────────────────────────────────────────────────────
        _ => "?",
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Date / time helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Return the localised weekday name for a chrono::Weekday.
pub fn weekday_name(lang: &Lang, weekday: chrono::Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => t(lang, "day.mon"),
        chrono::Weekday::Tue => t(lang, "day.tue"),
        chrono::Weekday::Wed => t(lang, "day.wed"),
        chrono::Weekday::Thu => t(lang, "day.thu"),
        chrono::Weekday::Fri => t(lang, "day.fri"),
        chrono::Weekday::Sat => t(lang, "day.sat"),
        chrono::Weekday::Sun => t(lang, "day.sun"),
    }
}

/// Return the localised month name for a month number (1–12).
pub fn month_name(lang: &Lang, month: u32) -> &'static str {
    match month {
        1  => t(lang, "month.1"),
        2  => t(lang, "month.2"),
        3  => t(lang, "month.3"),
        4  => t(lang, "month.4"),
        5  => t(lang, "month.5"),
        6  => t(lang, "month.6"),
        7  => t(lang, "month.7"),
        8  => t(lang, "month.8"),
        9  => t(lang, "month.9"),
        10 => t(lang, "month.10"),
        11 => t(lang, "month.11"),
        12 => t(lang, "month.12"),
        _  => "",
    }
}

/// Format a date in the localised style: "Lundi 14 janvier 2025" or "Monday 14 January 2025".
pub fn format_date<Tz: chrono::TimeZone>(lang: &Lang, dt: &chrono::DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    use chrono::Datelike;
    let wd   = weekday_name(lang, dt.weekday());
    let mo   = month_name(lang, dt.month());
    let day  = dt.day();
    let year = dt.year();
    match lang {
        // Day-of-week first, then day number, then month name, then year
        Lang::Fr | Lang::Es | Lang::Pt | Lang::It =>
            format!("{} {} {} {}", wd, day, mo, year),
        // English and German: weekday, month-name day, year
        Lang::En =>
            format!("{}, {} {} {}", wd, mo, day, year),
        Lang::De =>
            format!("{}, {} {} {}", wd, day, mo, year),
    }
}

/// Format a UTC event date as a short localised date string: "14 jan. 2025 10:30 UTC"
pub fn format_event_datetime(lang: &Lang, dt: &chrono::DateTime<chrono::Utc>) -> String {
    use chrono::Datelike;
    let mo   = month_name(lang, dt.month());
    let day  = dt.day();
    let year = dt.year();
    let time = dt.format("%H:%M UTC").to_string();
    match lang {
        Lang::En => format!("{} {} {}  ⏰ {}", mo, day, year, time),
        Lang::De => format!("{}. {}. {}  ⏰ {}", day, mo, year, time),
        _        => format!("{}. {} {}  ⏰ {}", day, mo, year, time),
    }
}

/// Return the localised moon phase name key given a phase value in [0, 1).
pub fn moon_phase_key(phase: f64) -> &'static str {
    match (phase * 8.0) as u32 {
        0 => "moon.new",
        1 => "moon.waxing_crescent",
        2 => "moon.first_quarter",
        3 => "moon.waxing_gibbous",
        4 => "moon.full",
        5 => "moon.waning_gibbous",
        6 => "moon.last_quarter",
        _ => "moon.waning_crescent",
    }
}
