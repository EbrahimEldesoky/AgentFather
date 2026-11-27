import { useEffect, useRef, useState } from 'react';

const COUNTRY_CODES = [
  { code: '+20', label: 'Egypt (+20)' },
  { code: '+966', label: 'Saudi Arabia (+966)' },
  { code: '+971', label: 'UAE (+971)' },
  { code: '+965', label: 'Kuwait (+965)' },
  { code: '+974', label: 'Qatar (+974)' },
  { code: '+962', label: 'Jordan (+962)' },
  { code: '+33', label: 'France (+33)' },
  { code: '+1', label: 'USA / Canada (+1)' },
  { code: '+44', label: 'UK (+44)' },
  { code: '+49', label: 'Germany (+49)' },
];

const COPY = {
  en: {
    navHome: 'Home',
    navFeatures: 'Features',
    navPricing: 'Pricing',
    navStatus: 'Status',
    navDash: 'Dashboard',
    login: 'Log In',
    signup: 'Sign Up',
    logout: 'Log Out',
    subtitle: 'Your personal AI executive agent — connecting your tools and executing tasks across WhatsApp and the web.',
    enterDash: 'Open Dashboard',
    getStarted: 'Get Started',
    integrationsTitle: 'Ecosystem & Integrations',
    integrationsSub: '30+ connectors across Business, Social Media, and Developer workflows.',
    business: 'Business',
    socialMedia: 'Social Media',
    developers: 'Developers',
    soon: 'SOON',
    connect: 'CONNECT',
    linked: 'LINKED',
    chatTitle: 'Agent Father',
    chatPlaceholder: 'Write a message to Agent Father...',
    send: 'Send',
    greeting: 'Hello! I am Agent Father. How can I help you today?',
    waModalTitle: 'Link WhatsApp',
    waStep1Desc: 'Enter your WhatsApp number. Agent Father will send you a verification code.',
    waSendCode: 'Send Verification Code',
    waStep2Desc: 'Enter the 6-digit code sent to your WhatsApp.',
    waVerifyBtn: 'Verify & Link',
    waLinkedSuccess: 'Successfully linked',
    linkWaBtn: 'Link WhatsApp',
    dashWelcome: 'Workspace',
    tabOverview: 'Overview',
    tabWorkspace: 'Workspace',
    tabChat: 'Chat',
    tabIntegrations: 'Integrations',
    tabAccount: 'Account',
    isolatedSession: 'Isolated Session',
    agentStatus: 'Agent Status',
    onlineReady: 'Online & Ready',
    notLinked: 'Not Linked',
    sessionMsgs: 'Session Messages',
    quickAction: 'Quick Action',
    quickActionDesc: 'Start chatting with Agent Father',
    workspaceSub: 'Create and manage your custom AI sub-agents.',
    newSubAgent: 'New Sub-Agent',
    noSubAgents: 'No sub-agents yet',
    noSubAgentsSub: 'Create your first sub-agent to get started.',
    agentName: 'Agent Name',
    agentNamePlaceholder: 'e.g. Research Assistant',
    agentRole: 'Role / Specialty',
    agentRolePlaceholder: 'e.g. Financial Analyst',
    agentSystemPrompt: 'Custom System Prompt',
    agentPromptPlaceholder: 'Describe how this agent should behave, its personality, tasks, and expertise...',
    createAgent: 'Create Sub-Agent',
    agentFormErr: 'Name and role are required.',
    chatWith: '💬 Chat with this agent',
    activeAgent: '✓ Active — click to deactivate',
    chatWithAgent: 'Chat started with',
    backToAgentFather: 'Back to Agent Father',
    createAgentInChat: 'Create Agent in Chat',
    workspaceChatSub: 'Ask Agent Father in Chat to create, maintain, or install skills for your workspace agents.',
    newSession: 'New Session',
    sessionsTitle: 'Sessions',
    noSessions: 'No saved sessions yet',
    maintainInChat: 'Maintain / Skills',
    skillsLabel: 'Skills',
    helpersLabel: 'Helpers',
    profileLabel: 'Isolated Profile',
    shareBtn: 'SHARE',
    shareTitle: 'Share Agent on WhatsApp',
    shareDesc: 'Enter a WhatsApp number. This agent will send a text & voice introduction explaining its capabilities.',
    shareSend: 'Send Intro & Voice Note',
    editPromptTitle: 'System Prompt',
    savePrompt: 'Save Changes',
    tabMonitoring: 'System Monitoring',
    monitoringTitle: 'System Monitoring',
    monitoringSub: 'Live animated avatars & real-time activity feed for every Workspace agent — showing what each agent is doing and has completed.',
    monitoringEmpty: 'Create an agent in Chat to preview its monitoring avatar slot here.',
    monitoringAvatarStatus: 'Animated Avatar & Live Activity Feed',
    nameLabel: 'Name:',
    emailLabel: 'Email / ID:',
    isolationNote: 'Your account data and agent memory are strictly isolated from other users.',
    featuresTitle: 'Agent Father Capabilities',
    featuresSub: 'Agent Father is built to be your autonomous personal assistant for messaging, task execution, and workflow automation.',
    featuresList: [
      { title: 'Native WhatsApp Execution', desc: 'Send and receive text & voice notes directly on WhatsApp with secure OTP verification.' },
      { title: 'Strict Multi-User Isolation', desc: 'Every user gets an isolated chat session and memory scope with zero cross-account bleed.' },
      { title: 'Fast Voice Synthesis', desc: 'Generates natural voice notes in Arabic, English, or French and delivers them as native PTT audio.' },
      { title: '30+ Upcoming Connectors', desc: 'Clear roadmap connecting Google Workspace, Social Media channels, and Developer tools.' },
    ],
    pricingTitle: 'Pricing Plans',
    pricingSub: 'Start free and scale as your automation grows.',
    plans: [
      {
        name: 'Starter',
        price: '$0',
        period: '/ month',
        features: ['Dashboard AI chat', '1 WhatsApp number link', 'Standard text & voice notes'],
        highlight: false,
      },
      {
        name: 'Pro',
        price: '$19',
        period: '/ month',
        features: ['Priority execution speed', 'Business integrations (as released)', 'Unlimited chat history', 'Advanced voice synthesis'],
        highlight: true,
      },
      {
        name: 'Enterprise',
        price: 'Custom',
        period: '',
        features: ['Isolated multi-seat team accounts', 'Custom developer integrations', 'Dedicated infrastructure & SLA'],
        highlight: false,
      },
    ],
    statusTitle: 'System Status',
    statusSub: 'Live operational status of Agent Father services.',
    statusItems: [
      { name: 'Agent Father Core Engine', status: 'OPERATIONAL' },
      { name: 'WhatsApp & Voice Gateway', status: 'OPERATIONAL' },
      { name: 'Dashboard & Multi-User Isolation', status: 'OPERATIONAL' },
      { name: 'Business, Social & Developer Connectors', status: 'COMING SOON' },
    ],
    googleSoon: 'Google Sign-In (Soon)',
  },
  fr: {
    navHome: 'Accueil',
    navFeatures: 'Fonctionnalités',
    navPricing: 'Tarifs',
    navStatus: 'Statut',
    navDash: 'Tableau de bord',
    login: 'Connexion',
    signup: 'Créer un compte',
    logout: 'Déconnexion',
    subtitle: 'Votre agent exécutif IA personnel — connecte vos outils et exécute vos tâches sur WhatsApp et le web.',
    enterDash: 'Ouvrir le tableau de bord',
    getStarted: 'Commencer',
    integrationsTitle: 'Écosystème & Intégrations',
    integrationsSub: 'Plus de 30 connecteurs pour Entreprise, Réseaux Sociaux et Développeurs.',
    business: 'Entreprise (Business)',
    socialMedia: 'Réseaux Sociaux',
    developers: 'Développeurs',
    soon: 'BIENTÔT',
    connect: 'CONNECTER',
    linked: 'CONNECTÉ',
    chatTitle: 'Agent Father',
    chatPlaceholder: 'Écrivez un message à Agent Father...',
    send: 'Envoyer',
    greeting: 'Bonjour ! Je suis Agent Father. Comment puis-je vous aider aujourd’hui ?',
    waModalTitle: 'Lier WhatsApp',
    waStep1Desc: 'Entrez votre numéro WhatsApp. Agent Father vous enverra un code de vérification.',
    waSendCode: 'Envoyer le code',
    waStep2Desc: 'Entrez le code à 6 chiffres reçu sur WhatsApp.',
    waVerifyBtn: 'Vérifier et lier',
    waLinkedSuccess: 'Lié avec succès',
    linkWaBtn: 'Lier WhatsApp',
    dashWelcome: 'Espace de travail',
    tabOverview: 'Aperçu',
    tabWorkspace: 'Espace de travail',
    tabChat: 'Discussion',
    tabIntegrations: 'Intégrations',
    tabAccount: 'Compte',
    isolatedSession: 'Session isolée',
    agentStatus: 'Statut de l’agent',
    onlineReady: 'En ligne et prêt',
    notLinked: 'Non lié',
    sessionMsgs: 'Messages de session',
    quickAction: 'Action rapide',
    quickActionDesc: 'Démarrer une discussion avec Agent Father',
    workspaceSub: 'Créez et gérez vos sous-agents IA personnalisés.',
    newSubAgent: 'Nouveau sous-agent',
    noSubAgents: 'Aucun sous-agent pour l’instant',
    noSubAgentsSub: 'Créez votre premier sous-agent pour commencer.',
    agentName: 'Nom de l’agent',
    agentNamePlaceholder: 'ex. Assistant de recherche',
    agentRole: 'Rôle / Spécialité',
    agentRolePlaceholder: 'ex. Analyste financier',
    agentSystemPrompt: 'Prompt système personnalisé',
    agentPromptPlaceholder: 'Décrivez le comportement, la personnalité et les tâches de cet agent...',
    createAgent: 'Créer le sous-agent',
    agentFormErr: 'Le nom et le rôle sont obligatoires.',
    chatWith: '💬 Discuter avec cet agent',
    activeAgent: '✓ Actif — cliquez pour désactiver',
    chatWithAgent: 'Discussion démarrée avec',
    backToAgentFather: 'Retour à Agent Father',
    createAgentInChat: 'Créer un agent dans le chat',
    workspaceChatSub: 'Demandez à Agent Father dans le chat de créer, maintenir ou installer des compétences pour vos agents.',
    newSession: 'Nouvelle session',
    sessionsTitle: 'Sessions',
    noSessions: 'Aucune session enregistrée',
    maintainInChat: 'Maintenance / Compétences',
    skillsLabel: 'Compétences',
    helpersLabel: 'Sous-agents',
    profileLabel: 'Profil isolé',
    shareBtn: 'SHARE',
    shareTitle: 'Partager sur WhatsApp',
    shareDesc: 'Entrez un numéro WhatsApp. Cet agent enverra une présentation texte et vocale.',
    shareSend: 'Envoyer présentation',
    editPromptTitle: 'Prompt Système',
    savePrompt: 'Enregistrer',
    tabMonitoring: 'Surveillance système',
    monitoringTitle: 'Surveillance du système',
    monitoringSub: 'Avatars animés et suivi en temps réel de l’activité de chaque agent — affichant ses tâches en cours et terminées.',
    monitoringEmpty: 'Créez un agent dans le chat pour prévisualiser son avatar ici.',
    monitoringAvatarStatus: 'Avatar animé et flux d’activité en direct',
    nameLabel: 'Nom :',
    emailLabel: 'E-mail / ID :',
    isolationNote: 'Vos données et la mémoire de votre agent sont strictement isolées des autres utilisateurs.',
    featuresTitle: 'Capacités d’Agent Father',
    featuresSub: 'Agent Father est conçu comme votre assistant personnel autonome pour la messagerie, l’exécution de tâches et l’automatisation.',
    featuresList: [
      { title: 'Exécution WhatsApp native', desc: 'Envoyez et recevez des messages texte et vocaux sur WhatsApp avec vérification OTP sécurisée.' },
      { title: 'Isolation multi-utilisateurs stricte', desc: 'Chaque utilisateur dispose d’une session et d’une mémoire isolées sans aucun croisement.' },
      { title: 'Synthèse vocale rapide', desc: 'Génère des notes vocales naturelles en anglais, français ou arabe en format audio natif.' },
      { title: '30+ connecteurs à venir', desc: 'Feuille de route claire connectant Google Workspace, Réseaux Sociaux et outils Développeurs.' },
    ],
    pricingTitle: 'Tarifs',
    pricingSub: 'Commencez gratuitement et évoluez selon vos besoins.',
    plans: [
      {
        name: 'Starter',
        price: '0 $',
        period: '/ mois',
        features: ['Discussion IA sur le tableau de bord', '1 numéro WhatsApp lié', 'Messages texte et vocaux standard'],
        highlight: false,
      },
      {
        name: 'Pro',
        price: '19 $',
        period: '/ mois',
        features: ['Exécution prioritaire', 'Intégrations Business (dès sortie)', 'Historique illimité', 'Synthèse vocale avancée'],
        highlight: true,
      },
      {
        name: 'Enterprise',
        price: 'Sur mesure',
        period: '',
        features: ['Comptes d’équipe isolés', 'Intégrations développeurs sur mesure', 'Infrastructure dédiée & SLA'],
        highlight: false,
      },
    ],
    statusTitle: 'État du système',
    statusSub: 'Statut opérationnel en direct des services Agent Father.',
    statusItems: [
      { name: 'Moteur principal Agent Father', status: 'OPERATIONAL' },
      { name: 'Passerelle WhatsApp & Audio', status: 'OPERATIONAL' },
      { name: 'Tableau de bord & Isolation', status: 'OPERATIONAL' },
      { name: 'Connecteurs Business, Social & Dev', status: 'COMING SOON' },
    ],
    googleSoon: 'Connexion Google (Bientôt)',
  },
  ar: {
    navHome: 'الرئيسية',
    navFeatures: 'المميزات',
    navPricing: 'الأسعار',
    navStatus: 'الحالة',
    navDash: 'لوحة التحكم',
    login: 'تسجيل الدخول',
    signup: 'إنشاء حساب',
    logout: 'تسجيل الخروج',
    subtitle: 'وكيلك الذكي الشخصي — يربط تطبيقاتك وينفذ مهامك فوراً عبر واتساب والويب.',
    enterDash: 'الدخول إلى لوحة التحكم',
    getStarted: 'ابدأ الآن',
    integrationsTitle: 'التكاملات والقنوات',
    integrationsSub: 'أكثر من 30 تكاملاً مقسمة بين الأعمال والتواصل الاجتماعي وأدوات المطورين.',
    business: 'الأعمال (Business)',
    socialMedia: 'التواصل الاجتماعي (Social Media)',
    developers: 'المطورون (Developers)',
    soon: 'قريباً',
    connect: 'ربط الآن',
    linked: 'مرتبط',
    chatTitle: 'Agent Father',
    chatPlaceholder: 'اكتب رسالتك إلى Agent Father...',
    send: 'إرسال',
    greeting: 'أهلاً بك. أنا Agent Father، كيف يمكنني مساعدتك اليوم؟',
    waModalTitle: 'ربط واتساب',
    waStep1Desc: 'أدخل رقم الواتساب ليصلك رمز التحقق من Agent Father.',
    waSendCode: 'إرسال رمز التحقق',
    waStep2Desc: 'أدخل الرمز المكون من 6 أرقام الذي وصلك على واتساب.',
    waVerifyBtn: 'تأكيد والربط',
    waLinkedSuccess: 'تم الربط بنجاح',
    linkWaBtn: 'ربط واتساب',
    dashWelcome: 'مساحة العمل',
    tabOverview: 'نظرة عامة',
    tabWorkspace: 'مساحة العمل',
    tabChat: 'المحادثة',
    tabIntegrations: 'التكاملات',
    tabAccount: 'الحساب',
    isolatedSession: 'جلسة خاصة معزولة',
    agentStatus: 'حالة الوكيل',
    onlineReady: 'متصل وجاهز',
    notLinked: 'غير مرتبط بعد',
    sessionMsgs: 'رسائل الجلسة',
    quickAction: 'إجراء سريع',
    quickActionDesc: 'ابدأ محادثة مع Agent Father',
    workspaceSub: 'أنشئ وأدر وكلاءك الذكيين الفرعيين المخصصين.',
    newSubAgent: '+ وكيل فرعي جديد',
    noSubAgents: 'لا يوجد وكلاء فرعيون بعد',
    noSubAgentsSub: 'أنشئ أول وكيل فرعي للبدء.',
    agentName: 'اسم الوكيل',
    agentNamePlaceholder: 'مثال: مساعد البحث',
    agentRole: 'الدور / التخصص',
    agentRolePlaceholder: 'مثال: محلل مالي',
    agentSystemPrompt: 'مطالبة النظام المخصصة',
    agentPromptPlaceholder: 'صف كيف يجب أن يتصرف هذا الوكيل، شخصيته، مهامه وتخصصه...',
    createAgent: 'إنشاء الوكيل الفرعي',
    agentFormErr: 'الاسم والدور مطلوبان.',
    chatWith: '💬 محادثة مع هذا الوكيل',
    activeAgent: '✓ نشط — انقر للإيقاف',
    chatWithAgent: 'بدأت المحادثة مع',
    backToAgentFather: 'العودة إلى Agent Father',
    createAgentInChat: 'إنشاء وكيل من الشات',
    workspaceChatSub: 'اطلب من Agent Father داخل الشات إنشاء أي وكيل أو صيانته أو تنزيل مهارات ووكلاء مساعدين له.',
    newSession: 'جلسة جديدة',
    sessionsTitle: 'سجل الجلسات',
    noSessions: 'لا توجد جلسات محفوظة بعد',
    maintainInChat: 'صيانة / مهارات',
    skillsLabel: 'المهارات',
    helpersLabel: 'الوكلاء المساعدون',
    profileLabel: 'بروفايل معزول',
    shareBtn: 'SHARE',
    shareTitle: 'مشاركة الوكيل عبر واتساب',
    shareDesc: 'أدخل كود الدولة ورقم الواتساب، وسيرسل الوكيل رسالة تعريفية وريكورد صوتي يشرح فيه مهاراته وكيف يمكنه المساعدة.',
    shareSend: 'إرسال الرسالة والريكورد',
    editPromptTitle: 'تعديل System Prompt',
    savePrompt: 'حفظ التعديلات',
    tabMonitoring: 'مراقبة النظام',
    monitoringTitle: 'مراقبة النظام (System Monitoring)',
    monitoringSub: 'أفاتار متحرك لكل وكيل يتم إنشاؤه في مساحة العمل يعرض نشاطه المباشر وما يفعله وما أنجزه.',
    monitoringEmpty: 'أنشئ وكيلاً من الشات ليظهر مكان الأفاتار الخاص به هنا.',
    monitoringAvatarStatus: 'أفاتار متحرك وسجل نشاط مباشر',
    nameLabel: 'الاسم:',
    emailLabel: 'البريد / المعرف:',
    isolationNote: 'بياناتك ومحادثاتك معزولة تماماً عن بقية المستخدمين.',
    featuresTitle: 'قدرات Agent Father',
    featuresSub: 'صُمم Agent Father ليكون وكيلك الشخصي المستقل لإدارة المراسلات، تنفيذ المهام، والربط مع أدوات عملك.',
    featuresList: [
      { title: 'تكامل واتساب المباشر', desc: 'استقبال وإرسال رسائل نصية وصوتية (Voice Notes) بسرعة فائقة مع تحقق آمن عبر OTP.' },
      { title: 'عزل كامل لكل مستخدم', desc: 'كل مستخدم يمتلك جلسة محادثة وذاكرة مستقلة تماماً دون أي تداخل بين الحسابات.' },
      { title: 'ردود صوتية ذكية', desc: 'توليد رسائل صوتية طبيعية باللهجة المصرية أو الإنجليزية أو الفرنسية وإرسالها كريكورد مباشر.' },
      { title: 'منصة قابلة للتوسع', desc: 'خريطة طريق واضحة لربط تطبيقات Google Workspace ومنصات التواصل وأدوات المطورين.' },
    ],
    pricingTitle: 'خطط الأسعار',
    pricingSub: 'ابدأ مجاناً وقم بالترقية عند الحاجة.',
    plans: [
      {
        name: 'المجانية (Starter)',
        price: '$0',
        period: '/ شهرياً',
        features: ['محادثة لوحة التحكم', 'ربط رقم واتساب واحد', 'رسائل نصية وصوتية أساسية'],
        highlight: false,
      },
      {
        name: 'الاحترافية (Pro)',
        price: '$19',
        period: '/ شهرياً',
        features: ['أولوية في سرعة التنفيذ', 'تكاملات الأعمال (فور توفرها)', 'سجل محادثات غير محدود', 'دعم صوتي متقدم'],
        highlight: true,
      },
      {
        name: 'الشركات (Enterprise)',
        price: 'مخصص',
        period: '',
        features: ['حسابات فريق متعددة معزولة', 'تكاملات مخصصة للمطورين', 'خوادم مخصصة وSLA'],
        highlight: false,
      },
    ],
    statusTitle: 'حالة النظام',
    statusSub: 'الحالة الحية لخدمات Agent Father.',
    statusItems: [
      { name: 'محرك Agent Father الرئيسي', status: 'OPERATIONAL' },
      { name: 'بوابة واتساب والرسائل الصوتية', status: 'OPERATIONAL' },
      { name: 'لوحة التحكم وعزل الجلسات', status: 'OPERATIONAL' },
      { name: 'تكاملات الأعمال والتواصل الاجتماعي والمطورين', status: 'COMING SOON' },
    ],
    googleSoon: 'تسجيل الدخول عبر Google (قريباً)',
  },
};

// ─── SVG Icons ───────────────────────────────────────────────────────────────

function IconWhatsApp() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30" fill="none">
      <circle cx="16" cy="16" r="16" fill="#25D366" />
      <path
        d="M23.5 8.5A10.4 10.4 0 0 0 16 5.5C10.2 5.5 5.5 10.2 5.5 16c0 1.9.5 3.7 1.4 5.3L5.5 26.5l5.4-1.4A10.4 10.4 0 0 0 16 26.5c5.8 0 10.5-4.7 10.5-10.5 0-2.8-1.1-5.4-3-7.5zM16 24.8c-1.6 0-3.2-.4-4.6-1.2l-.3-.2-3.2.8.9-3.1-.2-.3A8.7 8.7 0 0 1 7.3 16c0-4.8 3.9-8.7 8.7-8.7 2.3 0 4.5.9 6.1 2.6a8.6 8.6 0 0 1 2.6 6.1c0 4.8-3.9 8.8-8.7 8.8zm4.8-6.5c-.3-.1-1.6-.8-1.9-.9-.3-.1-.5-.1-.7.1s-.8 1-.9 1.2-.3.2-.6 0c-1.7-.8-2.8-1.5-3.9-3.3-.3-.5.3-.5.9-1.6.1-.2 0-.4-.1-.5-.1-.2-.7-1.6-1-2.2-.3-.6-.5-.5-.7-.5h-.6c-.2 0-.5.1-.8.4-.3.3-1 1-1 2.4s1 2.8 1.2 3c.1.1 2 3.1 4.9 4.3.7.3 1.2.5 1.6.6.7.2 1.3.2 1.8.1.6-.1 1.6-.6 1.9-1.3.2-.6.2-1.1.1-1.3-.2-.2-.4-.3-.7-.4z"
        fill="white"
      />
    </svg>
  );
}

function IconGmail() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect width="32" height="32" rx="6" fill="#fff" />
      <path d="M4 8h24L16 17z" fill="#EA4335" />
      <path d="M4 8v16h4V13z" fill="#C5221F" />
      <path d="M28 8v16h-4V13z" fill="#1A73E8" />
      <path d="M8 13l8 6 8-6v11H8z" fill="#F1F3F4" />
    </svg>
  );
}

function IconTelegram() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#229ED9" />
      <path d="M23.5 9L6.5 15.5l5 1.5 2 6 3-3.5 5 3.5 2-14z" fill="white" opacity=".95" />
    </svg>
  );
}

function IconGoogleDrive() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <path d="M5 24l4-7h14l4 7z" fill="#1E88E5" />
      <path d="M16 4L9 16h6l7-12z" fill="#43A047" />
      <path d="M23 16l-7-12-7 12 4 7 7-12 3 5z" fill="#FDD835" />
    </svg>
  );
}

function IconGoogleDocs() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect x="6" y="2" width="20" height="28" rx="3" fill="#1A73E8" />
      <path d="M20 2v6h6" fill="#A8C7FA" />
      <rect x="10" y="14" width="12" height="1.8" rx=".9" fill="white" />
      <rect x="10" y="18.5" width="12" height="1.8" rx=".9" fill="white" />
      <rect x="10" y="23" width="8" height="1.8" rx=".9" fill="white" />
    </svg>
  );
}

function IconGoogleSheets() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect x="6" y="2" width="20" height="28" rx="3" fill="#34A853" />
      <path d="M20 2v6h6" fill="#81C995" />
      <rect x="10" y="12" width="5" height="3" rx=".5" fill="white" opacity=".9" />
      <rect x="17" y="12" width="5" height="3" rx=".5" fill="white" opacity=".9" />
      <rect x="10" y="17" width="5" height="3" rx=".5" fill="white" opacity=".9" />
      <rect x="17" y="17" width="5" height="3" rx=".5" fill="white" opacity=".9" />
      <rect x="10" y="22" width="5" height="3" rx=".5" fill="white" opacity=".9" />
      <rect x="17" y="22" width="5" height="3" rx=".5" fill="white" opacity=".9" />
    </svg>
  );
}

function IconGoogleCalendar() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect x="4" y="6" width="24" height="22" rx="3" fill="white" />
      <rect x="4" y="6" width="24" height="7" rx="3" fill="#1A73E8" />
      <text x="16" y="24" textAnchor="middle" fontSize="11" fontWeight="bold" fill="#1A73E8">31</text>
    </svg>
  );
}

function IconGoogleMeet() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect x="2" y="8" width="20" height="16" rx="3" fill="#00897B" />
      <path d="M22 13l8-5v16l-8-5V13z" fill="#00BFA5" />
    </svg>
  );
}

function IconGoogleForms() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect x="6" y="2" width="20" height="28" rx="3" fill="#673AB7" />
      <path d="M20 2v6h6" fill="#B39DDB" />
      <rect x="10" y="13" width="12" height="1.8" rx=".9" fill="white" />
      <circle cx="11" cy="18" r="1.5" fill="white" />
      <rect x="14" y="17" width="8" height="1.8" rx=".9" fill="white" />
      <circle cx="11" cy="23" r="1.5" fill="white" />
      <rect x="14" y="22" width="8" height="1.8" rx=".9" fill="white" />
    </svg>
  );
}

function IconYouTube() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect width="32" height="32" rx="7" fill="#FF0000" />
      <polygon points="13,21 21,16 13,11" fill="white" />
    </svg>
  );
}

function IconFacebook() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#1877F2" />
      <path d="M21 10h-3c-.6 0-1 .4-1 1v3h4l-.5 4H17v10h-4V18h-3v-4h3v-3c0-3.3 2-5 5-5 1.4 0 3 .3 3 .3V10z" fill="white" />
    </svg>
  );
}

function IconInstagram() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <defs>
        <linearGradient id="ig2" x1="0%" y1="100%" x2="100%" y2="0%">
          <stop offset="0%" stopColor="#FFDC80" />
          <stop offset="35%" stopColor="#F77737" />
          <stop offset="65%" stopColor="#C13584" />
          <stop offset="100%" stopColor="#833AB4" />
        </linearGradient>
      </defs>
      <rect width="32" height="32" rx="8" fill="url(#ig2)" />
      <rect x="8" y="8" width="16" height="16" rx="4" fill="none" stroke="white" strokeWidth="1.8" />
      <circle cx="16" cy="16" r="4" fill="none" stroke="white" strokeWidth="1.8" />
      <circle cx="22" cy="10" r="1.2" fill="white" />
    </svg>
  );
}

function IconX() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#111" stroke="#333" strokeWidth="1" />
      <path d="M18.3 14.5L25 7h-1.6l-5.8 6.3L13 7H7.5l7 9.9L7.5 25H9l6.1-6.6L20.9 25h5.6l-7.2-10.5z" fill="white" />
    </svg>
  );
}

function IconTikTok() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#010101" stroke="#333" strokeWidth="1" />
      <path d="M21 7h-3v12a3 3 0 1 1-3-3v-3a6 6 0 1 0 6 6V13a9 9 0 0 0 5 1.5v-3A5 5 0 0 1 21 7z" fill="white" />
    </svg>
  );
}

function IconLinkedIn() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect width="32" height="32" rx="6" fill="#0A66C2" />
      <rect x="6" y="11" width="4" height="14" fill="white" />
      <circle cx="8" cy="7.5" r="2.5" fill="white" />
      <path d="M14 11h4v2s1-2 4-2c4 0 4 3 4 5v9h-4v-8c0-1.5-.5-2.5-2-2.5S18 16 18 17v8h-4z" fill="white" />
    </svg>
  );
}

function IconSnapchat() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#FFFC00" />
      <path d="M16 6c-3.3 0-5.5 2.4-5.5 5.5v2c-.5.1-1.5.3-2 .8-.5.4-.3.8.1.9.6.1 1.5.4 1.8 1.1.1.3.1.6-.1.9l-.2.4c-.6 1.1-1.5 2.4-3.1 2.7-.3.1-.3.5 0 .6.6.2 1.3.5 2 .6.1.4.3.8.7.8h.5c.4 0 .9-.1 1.5-.1 1 0 2 .5 3.3.5s2.3-.5 3.3-.5c.6 0 1.1.1 1.5.1h.5c.4 0 .6-.4.7-.8.7-.1 1.4-.4 2-.6.3-.1.3-.5 0-.6-1.6-.3-2.5-1.6-3.1-2.7l-.2-.4c-.2-.3-.2-.6-.1-.9.3-.7 1.2-1 1.8-1.1.4-.1.6-.5.1-.9-.5-.5-1.5-.7-2-.8v-2C21.5 8.4 19.3 6 16 6z" fill="#222" />
    </svg>
  );
}

function IconPinterest() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#E60023" />
      <path d="M16 7c-5 0-9 4-9 9 0 3.8 2.3 7.1 5.7 8.4-.1-.7-.1-1.8.2-2.6l1.2-5s-.3-.6-.3-1.5c0-1.4.8-2.4 1.9-2.4.9 0 1.3.7 1.3 1.5 0 .9-.6 2.3-.9 3.5-.3 1 .5 1.8 1.5 1.8 1.8 0 3-2.2 3-5 0-2-1.4-3.5-3.8-3.5-2.8 0-4.4 2.1-4.4 4.3 0 .8.3 1.4.6 1.8l-.2 1.1c-.1.3-.3.3-.5.2-1.4-.6-2-2.1-2-3.8 0-2.8 2.4-6.1 7-6.1 3.7 0 6.2 2.7 6.2 5.6 0 3.8-2.1 6.7-5.2 6.7-1 0-2-.5-2.3-1.2l-.6 2.4c-.2.8-.8 1.7-1.2 2.3.8.2 1.6.3 2.5.3 5 0 9-4 9-9s-4-9-9-9z" fill="white" />
    </svg>
  );
}

function IconReddit() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#FF4500" />
      <path d="M16 10c-3 0-5.5.6-7.3 1.6A3 3 0 0 0 5 14a3 3 0 0 0 1.5 2.6c0 .1-.1.3-.1.4 0 3.1 3.6 5.6 8 5.6s8-2.5 8-5.6c0-.1 0-.3-.1-.4A3 3 0 0 0 24 14a3 3 0 0 0-3.4-2.9A15 15 0 0 0 16 10zm-3.5 7a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm7 0a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z" fill="white" />
      <circle cx="21" cy="6" r="2" fill="white" />
      <line x1="21" y1="8" x2="18" y2="11" stroke="white" strokeWidth="1.5" />
    </svg>
  );
}

function IconDiscord() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#5865F2" />
      <path d="M23.6 9.6A19 19 0 0 0 19 8.3c-.2.3-.4.8-.5 1.2a17.9 17.9 0 0 0-5 0A13 13 0 0 0 13 8.3a19 19 0 0 0-4.6 1.3A19.7 19.7 0 0 0 5 22a19.1 19.1 0 0 0 5.8 2.7c.5-.6.9-1.3 1.2-2a12 12 0 0 1-1.9-.9l.5-.3a13.6 13.6 0 0 0 10.8 0l.5.3c-.6.4-1.2.7-1.9.9.3.7.7 1.4 1.2 2A19 19 0 0 0 27 22a19.7 19.7 0 0 0-3.4-12.4zm-10 10c-1 0-1.9-.9-1.9-2.1s.8-2.1 1.9-2.1 1.9.9 1.9 2.1-.8 2.1-1.9 2.1zm7 0c-1 0-1.9-.9-1.9-2.1s.8-2.1 1.9-2.1 1.9.9 1.9 2.1-.8 2.1-1.9 2.1z" fill="white" />
    </svg>
  );
}

function IconThreads() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#111" stroke="#333" strokeWidth="1" />
      <text x="16" y="21" textAnchor="middle" fontSize="16" fontWeight="bold" fill="white">@</text>
    </svg>
  );
}

function IconGitHub() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#24292E" />
      <path d="M16 5a11 11 0 0 0-3.5 21.4c.5.1.7-.2.7-.5v-1.7c-2.9.6-3.5-1.4-3.5-1.4-.5-1.2-1.1-1.5-1.1-1.5-.9-.6.1-.6.1-.6 1 .1 1.5 1 1.5 1 .9 1.5 2.3 1.1 2.9.8.1-.6.3-1.1.6-1.3-2.3-.3-4.7-1.2-4.7-5.2 0-1.1.4-2.1 1-2.8-.1-.3-.5-1.3.1-2.8 0 0 .9-.3 2.8 1 .8-.2 1.7-.3 2.5-.3s1.7.1 2.5.3c2-1.3 2.8-1 2.8-1 .6 1.5.2 2.5.1 2.8.6.7 1 1.7 1 2.8 0 4-2.4 4.9-4.7 5.2.4.3.7 1 .7 2v3c0 .3.2.6.7.5A11 11 0 0 0 16 5z" fill="white" />
    </svg>
  );
}

function IconVSCode() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect width="32" height="32" rx="6" fill="#007ACC" />
      <path d="M23 7l-9 8.5L8.5 11 7 12l5 4-5 4 1.5 1 6-4.5 9 8.5 2-1V8z" fill="white" />
    </svg>
  );
}

function IconSlack() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect width="32" height="32" rx="6" fill="#4A154B" />
      <path d="M13 7c0-1.1-.9-2-2-2s-2 .9-2 2v4c0 1.1.9 2 2 2s2-.9 2-2z" fill="#E01E5A" />
      <path d="M25 13c1.1 0 2-.9 2-2s-.9-2-2-2h-4c-1.1 0-2 .9-2 2s.9 2 2 2z" fill="#36C5F0" />
      <path d="M19 25c0 1.1.9 2 2 2s2-.9 2-2v-4c0-1.1-.9-2-2-2s-2 .9-2 2z" fill="#2EB67D" />
      <path d="M7 19c-1.1 0-2 .9-2 2s.9 2 2 2h4c1.1 0 2-.9 2-2s-.9-2-2-2z" fill="#ECB22E" />
    </svg>
  );
}

function IconJira() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect width="32" height="32" rx="6" fill="#0052CC" />
      <path d="M16 5l-9 9 4 4 5-5 5 5 4-4z" fill="#2684FF" />
      <path d="M16 15l-5 5 5 5 5-5z" fill="white" />
    </svg>
  );
}

function IconNotion() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect width="32" height="32" rx="6" fill="white" />
      <path d="M9 6h10l6 6v14H9z" fill="#F7F6F3" stroke="#37352F" strokeWidth="1.5" />
      <rect x="12" y="14" width="8" height="1.5" rx=".75" fill="#37352F" />
      <rect x="12" y="18" width="8" height="1.5" rx=".75" fill="#37352F" />
      <rect x="12" y="22" width="5" height="1.5" rx=".75" fill="#37352F" />
    </svg>
  );
}

function IconZapier() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#FF4A00" />
      <path d="M16 7v7H9l7 11v-7h7z" fill="white" />
    </svg>
  );
}

function IconMake() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#6D00CC" />
      <text x="16" y="21" textAnchor="middle" fontSize="14" fontWeight="bold" fill="white">M</text>
    </svg>
  );
}

function IconAWS() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect width="32" height="32" rx="6" fill="#232F3E" />
      <text x="16" y="18" textAnchor="middle" fontSize="10" fontWeight="bold" fill="#FF9900">AWS</text>
      <path d="M8 23l8 3 8-3" stroke="#FF9900" strokeWidth="1.8" fill="none" strokeLinecap="round" />
    </svg>
  );
}

function IconDocker() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <rect width="32" height="32" rx="6" fill="#2496ED" />
      <rect x="6" y="14" width="4" height="3" rx=".5" fill="white" />
      <rect x="11" y="14" width="4" height="3" rx=".5" fill="white" />
      <rect x="16" y="14" width="4" height="3" rx=".5" fill="white" />
      <rect x="11" y="10" width="4" height="3" rx=".5" fill="white" />
      <rect x="16" y="10" width="4" height="3" rx=".5" fill="white" />
      <path d="M5 19h22c-1 4-5 6-11 6S6 23 5 19z" fill="white" />
    </svg>
  );
}

function IconVercel() {
  return (
    <svg viewBox="0 0 32 32" width="30" height="30">
      <circle cx="16" cy="16" r="16" fill="#111" stroke="#333" strokeWidth="1" />
      <path d="M16 8l10 16H6z" fill="white" />
    </svg>
  );
}

// ─── Integration Tile Data (5x2 = 10 per category) ───────────────────────────

const BUSINESS_TILES = [
  { id: 'whatsapp', name: 'WhatsApp', icon: <IconWhatsApp />, action: 'connect' },
  { id: 'gmail', name: 'Gmail', icon: <IconGmail />, action: 'soon' },
  { id: 'telegram', name: 'Telegram', icon: <IconTelegram />, action: 'soon' },
  { id: 'gdrive', name: 'Google Drive', icon: <IconGoogleDrive />, action: 'soon' },
  { id: 'gdocs', name: 'Google Docs', icon: <IconGoogleDocs />, action: 'soon' },
  { id: 'gsheets', name: 'Google Sheets', icon: <IconGoogleSheets />, action: 'soon' },
  { id: 'gcalendar', name: 'Google Calendar', icon: <IconGoogleCalendar />, action: 'soon' },
  { id: 'gmeet', name: 'Google Meet', icon: <IconGoogleMeet />, action: 'soon' },
  { id: 'gforms', name: 'Google Forms', icon: <IconGoogleForms />, action: 'soon' },
  { id: 'youtube', name: 'YouTube', icon: <IconYouTube />, action: 'soon' },
];

const SOCIAL_TILES = [
  { id: 'facebook', name: 'Facebook', icon: <IconFacebook />, action: 'soon' },
  { id: 'instagram', name: 'Instagram', icon: <IconInstagram />, action: 'soon' },
  { id: 'x', name: 'X (Twitter)', icon: <IconX />, action: 'soon' },
  { id: 'tiktok', name: 'TikTok', icon: <IconTikTok />, action: 'soon' },
  { id: 'linkedin', name: 'LinkedIn', icon: <IconLinkedIn />, action: 'soon' },
  { id: 'snapchat', name: 'Snapchat', icon: <IconSnapchat />, action: 'soon' },
  { id: 'pinterest', name: 'Pinterest', icon: <IconPinterest />, action: 'soon' },
  { id: 'reddit', name: 'Reddit', icon: <IconReddit />, action: 'soon' },
  { id: 'discord', name: 'Discord', icon: <IconDiscord />, action: 'soon' },
  { id: 'threads', name: 'Threads', icon: <IconThreads />, action: 'soon' },
];

const DEV_TILES = [
  { id: 'github', name: 'GitHub', icon: <IconGitHub />, action: 'soon' },
  { id: 'vscode', name: 'VS Code', icon: <IconVSCode />, action: 'soon' },
  { id: 'slack', name: 'Slack', icon: <IconSlack />, action: 'soon' },
  { id: 'jira', name: 'Jira', icon: <IconJira />, action: 'soon' },
  { id: 'notion', name: 'Notion', icon: <IconNotion />, action: 'soon' },
  { id: 'zapier', name: 'Zapier', icon: <IconZapier />, action: 'soon' },
  { id: 'make', name: 'Make', icon: <IconMake />, action: 'soon' },
  { id: 'aws', name: 'AWS', icon: <IconAWS />, action: 'soon' },
  { id: 'docker', name: 'Docker', icon: <IconDocker />, action: 'soon' },
  { id: 'vercel', name: 'Vercel', icon: <IconVercel />, action: 'soon' },
];

// ─── Reusable Integration Section ────────────────────────────────────────────

function IntegrationCategories({ t, verifiedWa, onWaClick }) {
  const categories = [
    { key: 'biz', title: t.business, tiles: BUSINESS_TILES },
    { key: 'soc', title: t.socialMedia, tiles: SOCIAL_TILES },
    { key: 'dev', title: t.developers, tiles: DEV_TILES },
  ];

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '36px' }}>
      {categories.map((cat) => (
        <div key={cat.key}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '16px' }}>
            <h3 style={{ fontSize: '0.95rem', fontWeight: 700, letterSpacing: '0.06em', textTransform: 'uppercase' }}>
              {cat.title}
            </h3>
            <div style={{ flex: 1, height: '1px', background: 'var(--gray-dark)' }} />
          </div>
          <div className="integration-grid-5">
            {cat.tiles.map((tile) => {
              const isWa = tile.id === 'whatsapp';
              const isLinked = isWa && Boolean(verifiedWa);
              return (
                <div
                  key={tile.id}
                  onClick={isWa ? onWaClick : undefined}
                  className={`integration-tile ${isWa ? 'active-tile' : 'soon-tile'} ${isLinked ? 'linked-tile' : ''}`}
                >
                  {tile.icon}
                  <span className="tile-name">{tile.name}</span>
                  <span className={`tile-badge ${isWa ? 'live' : ''}`}>
                    {isLinked ? t.linked : isWa ? t.connect : t.soon}
                  </span>
                </div>
              );
            })}
          </div>
        </div>
      ))}
    </div>
  );
}

// ─── Main App ─────────────────────────────────────────────────────────────────

export default function App() {
  // Default language is ALWAYS English ('en'), followed by French ('fr'), then Arabic ('ar')
  const [lang, setLang] = useState('en');
  const langRef = useRef(lang);
  useEffect(() => {
    langRef.current = lang;
  }, [lang]);
  const t = COPY[lang];

  const [page, setPage] = useState('home');

  const [token, setToken] = useState(() => {
    const params = new URLSearchParams(window.location.search);
    const urlToken = params.get('token');
    if (urlToken) {
      localStorage.setItem('af_token', urlToken);
      window.history.replaceState({}, '', '/');
      return urlToken;
    }
    return localStorage.getItem('af_token') || '';
  });
  const [user, setUser] = useState(null);
  const [authModal, setAuthModal] = useState(null);
  const [authForm, setAuthForm] = useState({ name: '', email: '', password: '' });
  const [authMsg, setAuthMsg] = useState('');

  const [dashTab, setDashTab] = useState('workspace');

  // Sub-agents (Workspace) & Chat Sessions
  const [subAgents, setSubAgents] = useState([]);
  const [sessions, setSessions] = useState([]);
  const [activeSessionId, setActiveSessionId] = useState('');

  // Edit Prompt Modal
  const [editAgent, setEditAgent] = useState(null);
  const [editBusy, setEditBusy] = useState(false);

  // Share Agent Modal
  const [shareAgent, setShareAgent] = useState(null);
  const [shareCountryCode, setShareCountryCode] = useState('+20');
  const [sharePhone, setSharePhone] = useState('');
  const [shareBusy, setShareBusy] = useState(false);
  const [shareFeedback, setShareFeedback] = useState(null);

  // WhatsApp OTP Modal
  const [waModalOpen, setWaModalOpen] = useState(false);
  const [waStep, setWaStep] = useState(1);
  const [countryCode, setCountryCode] = useState('+20');
  const [phoneNumber, setPhoneNumber] = useState('');
  const [otpCode, setOtpCode] = useState('');
  const [waBusy, setWaBusy] = useState(false);
  const [waFeedback, setWaFeedback] = useState(null);
  const [verifiedWa, setVerifiedWa] = useState('');

  // Chat (per-user isolated)
  const [messages, setMessages] = useState([{ role: 'agent', text: COPY.en.greeting }]);
  const [chatInput, setChatInput] = useState('');
  const [chatBusy, setChatBusy] = useState(false);
  const [micState, setMicState] = useState('idle'); // 'idle' | 'listening' | 'transcribing'
  const chatEndRef = useRef(null);
  const recognitionRef = useRef(null);
  const mediaRecorderRef = useRef(null);
  const audioChunksRef = useRef([]);

  useEffect(() => {
    chatEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages, dashTab, page]);

  useEffect(() => {
    if (!token) return;
    fetch('/api/auth/me', { headers: { Authorization: `Bearer ${token}` } })
      .then((r) => (r.ok ? r.json() : Promise.reject()))
      .then((d) => {
        setUser(d.user);
        setPage('dashboard');
        if (d.user?.whatsapp_number) setVerifiedWa(`+${d.user.whatsapp_number}`);
        fetch('/api/chat/history', { headers: { Authorization: `Bearer ${token}` } })
          .then((hr) => (hr.ok ? hr.json() : null))
          .then((hd) => {
            if (hd?.messages && hd.messages.length > 0) {
              setMessages(hd.messages);
            } else {
              setMessages([{ role: 'agent', text: COPY[langRef.current].greeting }]);
            }
          })
          .catch(() => {});
        fetch('/api/sub-agents', { headers: { Authorization: `Bearer ${token}` } })
          .then((r) => (r.ok ? r.json() : null))
          .then((d) => { if (d?.sub_agents) setSubAgents(d.sub_agents); })
          .catch(() => {});
        fetch('/api/chat/sessions', { headers: { Authorization: `Bearer ${token}` } })
          .then((r) => (r.ok ? r.json() : null))
          .then((d) => { if (d?.sessions) setSessions(d.sessions); })
          .catch(() => {});
      })
      .catch(() => {
        localStorage.removeItem('af_token');
        setToken('');
        setUser(null);
      });
  }, [token]);

  async function handleAuthSubmit(e) {
    e.preventDefault();
    setAuthMsg('');
    const endpoint = authModal === 'signup' ? '/api/auth/signup' : '/api/auth/login';
    try {
      const res = await fetch(endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(authForm),
      });
      const data = await res.json();
      if (!res.ok) {
        setAuthMsg(data.error || 'Authentication failed');
        return;
      }
      localStorage.setItem('af_token', data.token);
      setToken(data.token);
      setUser(data.user);
      setAuthModal(null);
      setAuthForm({ name: '', email: '', password: '' });
      setPage('dashboard');
      setDashTab('workspace');
    } catch {
      setAuthMsg('Network error');
    }
  }

  function handleLogout() {
    fetch('/api/auth/logout', { method: 'POST' }).catch(() => {});
    localStorage.removeItem('af_token');
    setToken('');
    setUser(null);
    setVerifiedWa('');
    setMessages([{ role: 'agent', text: t.greeting }]);
    setSubAgents([]);
    setSessions([]);
    setActiveSessionId('');
    setPage('home');
  }

  async function refreshSubAgents() {
    if (!token) return;
    try {
      const r = await fetch('/api/sub-agents', { headers: { Authorization: `Bearer ${token}` } });
      if (r.ok) {
        const d = await r.json();
        setSubAgents(d.sub_agents || []);
      }
    } catch {
      // ignore
    }
  }

  async function loadSessions(saId) {
    if (!token) return;
    const qs = saId ? `?sub_agent_id=${encodeURIComponent(saId)}` : '';
    try {
      const r = await fetch(`/api/chat/sessions${qs}`, { headers: { Authorization: `Bearer ${token}` } });
      if (r.ok) {
        const d = await r.json();
        setSessions(d.sessions || []);
      }
    } catch {
      // ignore
    }
  }

  async function selectSession(sid) {
    setActiveSessionId(sid);
    if (!token) return;
    try {
      const r = await fetch(`/api/chat/history?session_id=${encodeURIComponent(sid)}`, {
        headers: { Authorization: `Bearer ${token}` },
      });
      if (r.ok) {
        const d = await r.json();
        if (d.messages && d.messages.length > 0) {
          setMessages(d.messages);
        } else {
          setMessages([{ role: 'agent', text: t.greeting }]);
        }
      }
    } catch {
      // ignore
    }
  }

  async function handleNewSession() {
    if (!token) return;
    try {
      const r = await fetch('/api/chat/sessions', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${token}` },
        body: JSON.stringify({ title: t.newSession }),
      });
      if (r.ok) {
        const d = await r.json();
        if (d.session) {
          setSessions((prev) => [d.session, ...prev]);
          setActiveSessionId(d.session.id);
          setMessages([{ role: 'agent', text: t.greeting }]);
        }
      }
    } catch {
      // ignore
    }
  }

  async function handleDeleteSession(sid) {
    if (!token) return;
    try {
      await fetch(`/api/chat/sessions/${encodeURIComponent(sid)}`, {
        method: 'DELETE',
        headers: { Authorization: `Bearer ${token}` },
      });
      setSessions((prev) => prev.filter((s) => s.id !== sid));
      if (activeSessionId === sid) {
        setActiveSessionId('');
        setMessages([{ role: 'agent', text: t.greeting }]);
      }
    } catch {
      // ignore
    }
  }

  async function handleSavePrompt(e) {
    e.preventDefault();
    if (!editAgent || editBusy) return;
    setEditBusy(true);
    try {
      const res = await fetch(`/api/sub-agents/${encodeURIComponent(editAgent.id)}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${token}` },
        body: JSON.stringify({
          name: editAgent.name,
          role: editAgent.role,
          system_prompt: editAgent.system_prompt,
        }),
      });
      const data = await res.json();
      if (res.ok && data.sub_agent) {
        setSubAgents((prev) => prev.map((a) => (a.id === data.sub_agent.id ? data.sub_agent : a)));
        setEditAgent(null);
      }
    } catch {
      // ignore
    } finally {
      setEditBusy(false);
    }
  }

  async function handleShareAgent(e) {
    e.preventDefault();
    if (!shareAgent || shareBusy) return;
    setShareBusy(true);
    setShareFeedback(null);
    try {
      const res = await fetch(`/api/sub-agents/${encodeURIComponent(shareAgent.id)}/share`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${token}` },
        body: JSON.stringify({ country_code: shareCountryCode, phone_number: sharePhone }),
      });
      const data = await res.json();
      if (res.ok) {
        setShareFeedback({ ok: true, text: `✓ ${data.sent_to}` });
        setTimeout(() => {
          setShareAgent(null);
          setSharePhone('');
          setShareFeedback(null);
        }, 1400);
      } else {
        setShareFeedback({ ok: false, text: data.error || 'Failed to share' });
      }
    } catch {
      setShareFeedback({ ok: false, text: 'Connection error' });
    } finally {
      setShareBusy(false);
    }
  }

  async function handleMicToggle() {
    if (micState === 'transcribing') return;

    if (micState === 'listening') {
      if (recognitionRef.current) {
        recognitionRef.current.stop();
        recognitionRef.current = null;
      }
      if (mediaRecorderRef.current && mediaRecorderRef.current.state !== 'inactive') {
        mediaRecorderRef.current.stop();
      }
      return;
    }

    const SpeechRec = window.SpeechRecognition || window.webkitSpeechRecognition;
    if (SpeechRec) {
      const rec = new SpeechRec();
      rec.lang = lang === 'ar' ? 'ar-EG' : lang === 'fr' ? 'fr-FR' : 'en-US';
      rec.interimResults = false;
      rec.maxAlternatives = 1;
      rec.onresult = (event) => {
        const transcript = event.results?.[0]?.[0]?.transcript || '';
        if (transcript.trim()) {
          setChatInput((prev) => (prev ? `${prev} ${transcript.trim()}` : transcript.trim()));
        }
      };
      rec.onerror = () => {
        setMicState('idle');
        recognitionRef.current = null;
      };
      rec.onend = () => {
        setMicState('idle');
        recognitionRef.current = null;
      };
      recognitionRef.current = rec;
      setMicState('listening');
      rec.start();
      return;
    }

    // Fallback for browsers without Web Speech API (e.g. Firefox): MediaRecorder + /api/chat/transcribe
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const recorder = new MediaRecorder(stream);
      audioChunksRef.current = [];
      recorder.ondataavailable = (ev) => {
        if (ev.data && ev.data.size > 0) audioChunksRef.current.push(ev.data);
      };
      recorder.onstop = async () => {
        stream.getTracks().forEach((tr) => tr.stop());
        const blob = new Blob(audioChunksRef.current, { type: recorder.mimeType || 'audio/webm' });
        if (blob.size === 0) {
          setMicState('idle');
          return;
        }
        setMicState('transcribing');
        try {
          const buf = await blob.arrayBuffer();
          let binary = '';
          const bytes = new Uint8Array(buf);
          for (let i = 0; i < bytes.byteLength; i++) binary += String.fromCharCode(bytes[i]);
          const b64 = btoa(binary);
          const res = await fetch('/api/chat/transcribe', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${token}` },
            body: JSON.stringify({ audio_base64: b64, mime_type: recorder.mimeType || 'audio/webm' }),
          });
          if (res.ok) {
            const data = await res.json();
            if (data.text) {
              setChatInput((prev) => (prev ? `${prev} ${data.text}` : data.text));
            }
          }
        } catch {
          // ignore
        } finally {
          setMicState('idle');
          mediaRecorderRef.current = null;
        }
      };
      mediaRecorderRef.current = recorder;
      setMicState('listening');
      recorder.start();
    } catch {
      setMicState('idle');
    }
  }

  function openDashboardOrLogin(tab = 'chat') {
    if (user) {
      setDashTab(tab);
      setPage('dashboard');
    } else {
      setAuthModal('login');
    }
  }

  async function handleRequestOtp(e) {
    e.preventDefault();
    setWaBusy(true);
    setWaFeedback(null);
    try {
      const res = await fetch('/api/whatsapp/request-otp', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify({ country_code: countryCode, phone_number: phoneNumber, include_voice: true }),
      });
      const data = await res.json();
      if (res.ok) {
        setWaStep(2);
        setWaFeedback({ ok: true, text: data.message });
      } else {
        setWaFeedback({ ok: false, text: data.error || 'Failed to send code' });
      }
    } catch {
      setWaFeedback({ ok: false, text: 'Connection error' });
    } finally {
      setWaBusy(false);
    }
  }

  async function handleVerifyOtp(e) {
    e.preventDefault();
    setWaBusy(true);
    setWaFeedback(null);
    try {
      const res = await fetch('/api/whatsapp/verify-otp', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify({ country_code: countryCode, phone_number: phoneNumber, otp_code: otpCode }),
      });
      const data = await res.json();
      if (res.ok) {
        setVerifiedWa(data.verified_number);
        setWaFeedback({ ok: true, text: `${t.waLinkedSuccess}: ${data.verified_number}` });
        setTimeout(() => {
          setWaModalOpen(false);
          setWaStep(1);
          setOtpCode('');
        }, 1200);
      } else {
        setWaFeedback({ ok: false, text: data.error || 'Invalid code' });
      }
    } catch {
      setWaFeedback({ ok: false, text: 'Connection error' });
    } finally {
      setWaBusy(false);
    }
  }

  async function handleSendChat(e) {
    e.preventDefault();
    if (!token) {
      setAuthModal('login');
      return;
    }
    const text = chatInput.trim();
    if (!text || chatBusy) return;
    setChatInput('');
    setMessages((prev) => [...prev, { role: 'user', text }]);
    setChatBusy(true);
    try {
      const res = await fetch('/api/chat', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({
          message: text,
          session_id: activeSessionId || null,
        }),
      });
      const data = await res.json();
      if (data.session_id) {
        setActiveSessionId(data.session_id);
        loadSessions('');
      }
      if (data.agent_created || data.agent_updated) {
        refreshSubAgents();
      }
      setMessages((prev) => [
        ...prev,
        {
          role: 'agent',
          text: data.reply || data.error || 'No response',
          audio_url: data.audio_url || null,
        },
      ]);
    } catch {
      setMessages((prev) => [...prev, { role: 'agent', text: 'Connection error' }]);
    } finally {
      setChatBusy(false);
    }
  }

  return (
    <div dir={lang === 'ar' ? 'rtl' : 'ltr'}>
      {/* Navbar */}
      <header className="navbar">
        <div className="container navbar-inner">
          <a href="#" className="brand" onClick={(e) => { e.preventDefault(); setPage('home'); }}>
            <img src="/brand-assets/logo.png" alt="Agent Father" />
            <span>AGENT FATHER</span>
          </a>

          <nav className="nav-links">
            <button type="button" className={`nav-link-btn ${page === 'home' ? 'active' : ''}`} onClick={() => setPage('home')}>
              {t.navHome}
            </button>
            <button type="button" className={`nav-link-btn ${page === 'features' ? 'active' : ''}`} onClick={() => setPage('features')}>
              {t.navFeatures}
            </button>
            <button type="button" className={`nav-link-btn ${page === 'pricing' ? 'active' : ''}`} onClick={() => setPage('pricing')}>
              {t.navPricing}
            </button>
            <button type="button" className={`nav-link-btn ${page === 'status' ? 'active' : ''}`} onClick={() => setPage('status')}>
              {t.navStatus}
            </button>
            {user && (
              <button type="button" className={`nav-link-btn ${page === 'dashboard' ? 'active' : ''}`} onClick={() => setPage('dashboard')}>
                {t.navDash}
              </button>
            )}
          </nav>

          <div className="nav-actions">
            {/* Language Order: EN -> FR -> AR */}
            <div style={{ display: 'flex', border: '1px solid var(--gray-dark)', borderRadius: '4px', overflow: 'hidden' }}>
              {['en', 'fr', 'ar'].map((code) => (
                <button
                  key={code}
                  type="button"
                  onClick={() => setLang(code)}
                  style={{
                    padding: '6px 9px',
                    fontSize: '0.75rem',
                    fontWeight: lang === code ? 700 : 500,
                    background: lang === code ? 'var(--accent-red-primary)' : 'transparent',
                    color: lang === code ? 'var(--text-white)' : 'var(--highlight-silver)',
                    border: 'none',
                    cursor: 'pointer',
                  }}
                >
                  {code.toUpperCase()}
                </button>
              ))}
            </div>

            {user ? (
              <>
                <span className="nav-user-badge">{user.name}</span>
                <button type="button" className="btn btn-ghost" onClick={handleLogout}>{t.logout}</button>
              </>
            ) : (
              <>
                <button type="button" className="btn btn-ghost" onClick={() => setAuthModal('login')}>{t.login}</button>
                <button type="button" className="btn btn-primary" onClick={() => setAuthModal('signup')}>{t.signup}</button>
              </>
            )}
          </div>
        </div>
      </header>

      {/* ── HOME PAGE (Public — NO CHAT!) ── */}
      {page === 'home' && (
        <>
          <section className="hero-full">
            <div className="hero-banner-wrap">
              <img className="hero-banner-img" src="/brand-assets/main.png?v=1795736206" alt="Agent Father" />
            </div>
            <div className="hero-under">
              <h1>AGENT FATHER</h1>
              <p>{t.subtitle}</p>
              <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap', justifyContent: 'center' }}>
                <button
                  type="button"
                  className="btn btn-primary"
                  style={{ fontSize: '0.95rem', padding: '12px 28px' }}
                  onClick={() => openDashboardOrLogin('chat')}
                >
                  {user ? t.enterDash : t.getStarted}
                </button>
                <button
                  type="button"
                  className="btn btn-ghost"
                  style={{ fontSize: '0.95rem', padding: '12px 24px' }}
                  onClick={() => setPage('features')}
                >
                  {t.navFeatures}
                </button>
              </div>
            </div>
          </section>

          {/* 3-Category Integrations Showcase */}
          <section style={{ padding: '64px 0', borderBottom: '1px solid var(--gray-dark)' }}>
            <div className="container">
              <div style={{ textAlign: 'center', marginBottom: '44px' }}>
                <h2 style={{ fontSize: '1.7rem', fontWeight: 800, letterSpacing: '0.06em', marginBottom: '10px' }}>
                  {t.integrationsTitle}
                </h2>
                <p style={{ color: 'var(--highlight-silver)', fontSize: '0.95rem' }}>
                  {t.integrationsSub}
                </p>
              </div>
              <IntegrationCategories
                t={t}
                verifiedWa={verifiedWa}
                onWaClick={() => {
                  if (!user) setAuthModal('login');
                  else { setWaFeedback(null); setWaModalOpen(true); }
                }}
              />
            </div>
          </section>
        </>
      )}

      {/* ── FEATURES PAGE ── */}
      {page === 'features' && (
        <section style={{ padding: '64px 0', minHeight: '75vh', borderBottom: '1px solid var(--gray-dark)' }}>
          <div className="container">
            <h2 style={{ fontSize: '1.8rem', fontWeight: 800, marginBottom: '12px' }}>
              {t.featuresTitle}
            </h2>
            <p style={{ color: 'var(--highlight-silver)', marginBottom: '40px', maxWidth: '680px' }}>
              {t.featuresSub}
            </p>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))', gap: '20px' }}>
              {t.featuresList.map((item) => (
                <div key={item.title} style={{ background: 'var(--surface-charcoal)', border: '1px solid var(--gray-dark)', padding: '26px', borderRadius: '6px' }}>
                  <h3 style={{ fontSize: '1.05rem', fontWeight: 700, marginBottom: '10px', color: 'var(--text-white)' }}>{item.title}</h3>
                  <p style={{ fontSize: '0.88rem', color: 'var(--highlight-silver)', lineHeight: 1.65 }}>{item.desc}</p>
                </div>
              ))}
            </div>
          </div>
        </section>
      )}

      {/* ── PRICING PAGE ── */}
      {page === 'pricing' && (
        <section style={{ padding: '64px 0', minHeight: '75vh', borderBottom: '1px solid var(--gray-dark)' }}>
          <div className="container">
            <div style={{ textAlign: 'center', marginBottom: '44px' }}>
              <h2 style={{ fontSize: '1.8rem', fontWeight: 800, marginBottom: '10px' }}>
                {t.pricingTitle}
              </h2>
              <p style={{ color: 'var(--highlight-silver)', fontSize: '0.95rem' }}>
                {t.pricingSub}
              </p>
            </div>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))', gap: '20px', maxWidth: '960px', margin: '0 auto' }}>
              {t.plans.map((plan) => (
                <div
                  key={plan.name}
                  style={{
                    background: 'var(--surface-charcoal)',
                    border: `1px solid ${plan.highlight ? 'var(--accent-red-bright)' : 'var(--gray-dark)'}`,
                    padding: '28px',
                    borderRadius: '6px',
                    display: 'flex',
                    flexDirection: 'column',
                    justifyContent: 'space-between',
                    gap: '20px',
                  }}
                >
                  <div>
                    <div style={{ fontSize: '0.95rem', fontWeight: 700, marginBottom: '12px' }}>{plan.name}</div>
                    <div style={{ fontSize: '2rem', fontWeight: 800, marginBottom: '18px' }}>
                      {plan.price} <span style={{ fontSize: '0.85rem', color: 'var(--highlight-silver)', fontWeight: 400 }}>{plan.period}</span>
                    </div>
                    <ul style={{ listStyle: 'none', display: 'flex', flexDirection: 'column', gap: '10px', fontSize: '0.88rem', color: 'var(--highlight-silver)' }}>
                      {plan.features.map((f) => <li key={f}>• {f}</li>)}
                    </ul>
                  </div>
                  <button
                    type="button"
                    className={`btn ${plan.highlight ? 'btn-primary' : 'btn-ghost'}`}
                    onClick={() => openDashboardOrLogin('chat')}
                  >
                    {t.getStarted}
                  </button>
                </div>
              ))}
            </div>
          </div>
        </section>
      )}

      {/* ── STATUS PAGE ── */}
      {page === 'status' && (
        <section style={{ padding: '64px 0', minHeight: '75vh', borderBottom: '1px solid var(--gray-dark)' }}>
          <div className="container" style={{ maxWidth: '760px' }}>
            <h2 style={{ fontSize: '1.8rem', fontWeight: 800, marginBottom: '12px' }}>
              {t.statusTitle}
            </h2>
            <p style={{ color: 'var(--highlight-silver)', marginBottom: '32px' }}>
              {t.statusSub}
            </p>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
              {t.statusItems.map((s) => (
                <div key={s.name} style={{ background: 'var(--surface-charcoal)', border: '1px solid var(--gray-dark)', padding: '16px 20px', borderRadius: '6px', display: 'flex', justifyContent: 'space-between', alignItems: 'center', gap: '12px' }}>
                  <span style={{ fontWeight: 600, fontSize: '0.92rem' }}>{s.name}</span>
                  <span style={{ fontSize: '0.72rem', fontWeight: 700, letterSpacing: '0.08em', color: s.status === 'OPERATIONAL' ? '#25D366' : 'var(--gray-light)' }}>
                    {s.status}
                  </span>
                </div>
              ))}
            </div>
          </div>
        </section>
      )}

      {/* ── DASHBOARD (Logged-In Only — Includes Chat + Organized Workspace) ── */}
      {page === 'dashboard' && user && (
        <section style={{ minHeight: 'calc(100vh - 68px)', background: 'var(--bg-pure-black)', borderBottom: '1px solid var(--gray-dark)' }}>
          <div className="container" style={{ paddingTop: '32px', paddingBottom: '56px' }}>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '24px', flexWrap: 'wrap', gap: '12px' }}>
              <div>
                <div style={{ color: 'var(--highlight-silver)', fontSize: '0.78rem', letterSpacing: '0.08em', textTransform: 'uppercase', marginBottom: '4px' }}>
                  {t.dashWelcome}
                </div>
                <h2 style={{ fontSize: '1.45rem', fontWeight: 800 }}>{user.name}</h2>
              </div>
              <div style={{ display: 'flex', alignItems: 'center', gap: '10px', flexWrap: 'wrap' }}>
                <button
                  type="button"
                  className="btn btn-ghost"
                  onClick={() => { setWaFeedback(null); setWaModalOpen(true); }}
                >
                  {verifiedWa ? `WhatsApp: ${verifiedWa}` : t.linkWaBtn}
                </button>
              </div>
            </div>

            <div style={{ display: 'flex', gap: '4px', borderBottom: '1px solid var(--gray-dark)', marginBottom: '28px', overflowX: 'auto' }}>
              {[
                { key: 'workspace', label: t.tabWorkspace },
                { key: 'chat', label: t.tabChat },
                { key: 'integrations', label: t.tabIntegrations },
                { key: 'monitoring', label: t.tabMonitoring },
              ].map((tab) => (
                <button
                  key={tab.key}
                  type="button"
                  onClick={() => setDashTab(tab.key)}
                  className={`dash-tab-btn ${dashTab === tab.key ? 'active' : ''}`}
                >
                  {tab.label}
                </button>
              ))}
            </div>

            {dashTab === 'workspace' && (
              <div>
                {/* Sub-agent cards grid — created & maintained purely via Chat */}
                {subAgents.length === 0 ? (
                  <div style={{ textAlign: 'center', padding: '72px 0', color: 'var(--highlight-silver)' }}>
                    <div style={{ fontSize: '0.95rem' }}>{t.workspaceChatSub}</div>
                  </div>
                ) : (
                  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))', gap: '16px' }}>
                    {subAgents.map((sa) => (
                      <div
                        key={sa.id}
                        style={{
                          background: 'var(--surface-charcoal)',
                          border: '1px solid var(--gray-dark)',
                          borderRadius: '8px',
                          padding: '20px',
                          display: 'flex',
                          flexDirection: 'column',
                          gap: '12px',
                        }}
                      >
                        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '8px' }}>
                          <div style={{ fontSize: '1.05rem', fontWeight: 700 }}>{sa.name}</div>
                          <button
                            type="button"
                            onClick={async () => {
                              await fetch(`/api/sub-agents/${sa.id}`, { method: 'DELETE', headers: { Authorization: `Bearer ${token}` } });
                              setSubAgents((prev) => prev.filter((s) => s.id !== sa.id));
                            }}
                            style={{ background: 'none', border: 'none', cursor: 'pointer', color: 'var(--highlight-silver)', fontSize: '0.78rem', flexShrink: 0 }}
                            title="Delete"
                          >
                            🗑️
                          </button>
                        </div>

                        <div style={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'space-between', gap: '8px', background: 'rgba(255,255,255,0.02)', border: '1px solid var(--gray-dark)', borderRadius: '6px', padding: '10px 12px' }}>
                          <p style={{ fontSize: '0.82rem', color: 'var(--highlight-silver)', margin: 0, lineHeight: 1.55, flex: 1 }}>
                            {sa.system_prompt
                              ? (sa.system_prompt.length > 120 ? sa.system_prompt.slice(0, 120) + '…' : sa.system_prompt)
                              : '—'}
                          </p>
                          <button
                            type="button"
                            onClick={() => setEditAgent({ ...sa })}
                            style={{ background: 'none', border: 'none', cursor: 'pointer', color: 'var(--highlight-silver)', fontSize: '0.9rem', flexShrink: 0, padding: '2px' }}
                            title={t.editPromptTitle}
                          >
                            ✏️
                          </button>
                        </div>

                        {Array.isArray(sa.skills) && sa.skills.length > 0 && (
                          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', alignItems: 'center' }}>
                            <span style={{ fontSize: '0.7rem', color: 'var(--gray-light)' }}>{t.skillsLabel}:</span>
                            {sa.skills.map((sk) => (
                              <span key={sk} style={{ fontSize: '0.7rem', padding: '2px 7px', borderRadius: '4px', background: 'var(--bg-pure-black)', border: '1px solid var(--gray-dark)', color: 'var(--highlight-silver)' }}>
                                {sk}
                              </span>
                            ))}
                          </div>
                        )}

                        {Array.isArray(sa.helpers) && sa.helpers.length > 0 && (
                          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', alignItems: 'center' }}>
                            <span style={{ fontSize: '0.7rem', color: 'var(--gray-light)' }}>{t.helpersLabel}:</span>
                            {sa.helpers.map((hp, idx) => (
                              <span key={hp.id || idx} style={{ fontSize: '0.7rem', padding: '2px 7px', borderRadius: '4px', background: 'rgba(255,255,255,0.04)', border: '1px solid var(--gray-dark)', color: 'var(--text-white)' }}>
                                {hp.name}
                              </span>
                            ))}
                          </div>
                        )}

                        <button
                          type="button"
                          className="btn btn-primary"
                          style={{ marginTop: 'auto', width: '100%', fontSize: '0.84rem', letterSpacing: '0.06em' }}
                          onClick={() => { setShareFeedback(null); setSharePhone(''); setShareAgent(sa); }}
                        >
                          {t.shareBtn}
                        </button>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}

            {dashTab === 'chat' && (
              <div style={{ display: 'grid', gridTemplateColumns: 'minmax(220px, 260px) 1fr', gap: '16px', alignItems: 'stretch' }}>
                {/* Sessions Sidebar */}
                <div style={{ background: 'var(--surface-charcoal)', border: '1px solid var(--gray-dark)', borderRadius: '8px', padding: '14px', display: 'flex', flexDirection: 'column', gap: '10px', height: '580px' }}>
                  <button
                    type="button"
                    className="btn btn-primary"
                    style={{ width: '100%', fontSize: '0.84rem' }}
                    onClick={() => handleNewSession()}
                  >
                    + {t.newSession}
                  </button>
                  <div style={{ fontSize: '0.72rem', fontWeight: 700, color: 'var(--gray-light)', textTransform: 'uppercase', letterSpacing: '0.06em', marginTop: '4px' }}>
                    {t.sessionsTitle}
                  </div>
                  <div style={{ flex: 1, overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: '6px' }}>
                    {sessions.length === 0 ? (
                      <div style={{ fontSize: '0.78rem', color: 'var(--gray-light)', padding: '12px 4px' }}>
                        {t.noSessions}
                      </div>
                    ) : (
                      sessions.map((s) => (
                        <div
                          key={s.id}
                          style={{
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'space-between',
                            gap: '6px',
                            padding: '8px 10px',
                            borderRadius: '6px',
                            cursor: 'pointer',
                            background: activeSessionId === s.id ? 'rgba(220,38,38,0.16)' : 'rgba(255,255,255,0.03)',
                            border: `1px solid ${activeSessionId === s.id ? 'var(--accent-red-primary)' : 'transparent'}`,
                          }}
                          onClick={() => selectSession(s.id)}
                        >
                          <span style={{ fontSize: '0.8rem', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', flex: 1 }}>
                            {s.title}
                          </span>
                          <button
                            type="button"
                            onClick={(e) => { e.stopPropagation(); handleDeleteSession(s.id); }}
                            style={{ background: 'none', border: 'none', color: 'var(--gray-light)', cursor: 'pointer', fontSize: '0.72rem', flexShrink: 0 }}
                            title="Delete"
                          >
                            ✕
                          </button>
                        </div>
                      ))
                    )}
                  </div>
                </div>

                {/* Main Chat Panel */}
                <div className="chat-panel" style={{ height: '580px' }}>
                  <div className="chat-head">
                    <span className="chat-head-title">{t.chatTitle}</span>
                    <span style={{ fontSize: '0.75rem', color: '#25D366' }}>
                      ● {t.isolatedSession}
                    </span>
                  </div>
                  <div className="chat-messages">
                    {messages.map((m, i) => (
                      <div key={i} className={`msg ${m.role}`}>
                        {m.text && <div style={{ whiteSpace: 'pre-wrap' }}>{m.text}</div>}
                        {m.audio_url && (
                          <audio
                            controls
                            src={m.audio_url}
                            style={{ marginTop: m.text ? 8 : 0, maxWidth: '100%', height: 36 }}
                          />
                        )}
                      </div>
                    ))}
                    {chatBusy && <div className="msg agent" style={{ opacity: 0.6 }}>...</div>}
                    <div ref={chatEndRef} />
                  </div>
                  <form className="chat-input-row" onSubmit={handleSendChat}>
                    <input
                      className="input"
                      placeholder={t.chatPlaceholder}
                      value={chatInput}
                      onChange={(e) => setChatInput(e.target.value)}
                    />
                    <button
                      type="button"
                      className={`btn ${micState === 'listening' ? 'btn-primary' : 'btn-ghost'}`}
                      onClick={handleMicToggle}
                      disabled={chatBusy || micState === 'transcribing'}
                      title={micState === 'listening' ? 'Stop recording' : 'Voice input'}
                      style={{ minWidth: '44px', padding: '0 12px', fontSize: '1rem' }}
                    >
                      {micState === 'listening' ? '⏹️' : micState === 'transcribing' ? '⏳' : '🎤'}
                    </button>
                    <button className="btn btn-primary" type="submit" disabled={chatBusy}>
                      {chatBusy ? '...' : t.send}
                    </button>
                  </form>
                </div>
              </div>
            )}

            {dashTab === 'integrations' && (
              <IntegrationCategories
                t={t}
                verifiedWa={verifiedWa}
                onWaClick={() => { setWaFeedback(null); setWaModalOpen(true); }}
              />
            )}

            {dashTab === 'monitoring' && (
              <div>
                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', flexWrap: 'wrap', gap: '12px', marginBottom: '24px' }}>
                  <div>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '10px', marginBottom: '6px' }}>
                      <h3 style={{ fontSize: '1.15rem', fontWeight: 800 }}>{t.monitoringTitle}</h3>
                      <span style={{ padding: '2px 10px', borderRadius: '20px', background: 'rgba(220,38,38,0.16)', border: '1px solid var(--accent-red-primary)', color: 'var(--accent-red-bright)', fontSize: '0.7rem', fontWeight: 800, letterSpacing: '0.08em' }}>
                        {t.soon}
                      </span>
                    </div>
                    <p style={{ fontSize: '0.85rem', color: 'var(--highlight-silver)', maxWidth: '680px', lineHeight: 1.6 }}>
                      {t.monitoringSub}
                    </p>
                  </div>
                </div>

                {subAgents.length === 0 ? (
                  <div style={{ background: 'var(--surface-charcoal)', border: '1px dashed var(--gray-dark)', borderRadius: '8px', padding: '56px 24px', textAlign: 'center', color: 'var(--highlight-silver)' }}>
                    <div style={{ fontSize: '2.2rem', marginBottom: '10px' }}>🛰️</div>
                    <div style={{ fontSize: '0.92rem' }}>{t.monitoringEmpty}</div>
                  </div>
                ) : (
                  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))', gap: '16px' }}>
                    {subAgents.map((sa) => (
                      <div
                        key={sa.id}
                        style={{
                          background: 'var(--surface-charcoal)',
                          border: '1px solid var(--gray-dark)',
                          borderRadius: '8px',
                          padding: '20px',
                          display: 'flex',
                          flexDirection: 'column',
                          gap: '12px',
                          opacity: 0.88,
                        }}
                      >
                        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '10px' }}>
                          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                            <div style={{ width: '40px', height: '40px', borderRadius: '50%', background: 'rgba(220,38,38,0.15)', border: '1px solid var(--accent-red-primary)', display: 'flex', alignItems: 'center', justifyContent: 'center', fontSize: '1.2rem' }}>
                              🤖
                            </div>
                            <div style={{ fontWeight: 700, fontSize: '0.95rem' }}>{sa.name}</div>
                          </div>
                          <span style={{ fontSize: '0.68rem', fontWeight: 800, letterSpacing: '0.08em', color: 'var(--gray-light)' }}>
                            {t.soon}
                          </span>
                        </div>
                        <div style={{ fontSize: '0.78rem', color: 'var(--highlight-silver)', borderTop: '1px solid var(--gray-dark)', paddingTop: '10px' }}>
                          ● {t.monitoringAvatarStatus}
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}
          </div>
        </section>
      )}

      {/* Footer */}
      <footer className="footer">
        <div className="container footer-inner">
          <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
            <img src="/brand-assets/logo.png" alt="Logo" width={22} height={22} />
            <span>AGENT FATHER</span>
          </div>
          <span>© 2026 Agent Father</span>
        </div>
      </footer>

      {/* Edit Agent System Prompt Modal */}
      {editAgent && (
        <div className="modal-backdrop" onClick={() => setEditAgent(null)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-head">
              <h3>{editAgent.name} — {t.editPromptTitle}</h3>
              <button type="button" className="btn btn-ghost" onClick={() => setEditAgent(null)}>X</button>
            </div>
            <form className="form-stack" onSubmit={handleSavePrompt}>
              <textarea
                className="input"
                rows={7}
                value={editAgent.system_prompt || ''}
                onChange={(e) => setEditAgent({ ...editAgent, system_prompt: e.target.value })}
                style={{ resize: 'vertical', fontFamily: 'inherit', lineHeight: 1.5 }}
                required
              />
              <button className="btn btn-primary" type="submit" disabled={editBusy}>
                {editBusy ? '...' : t.savePrompt}
              </button>
            </form>
          </div>
        </div>
      )}

      {/* Share Agent via WhatsApp Modal */}
      {shareAgent && (
        <div className="modal-backdrop" onClick={() => setShareAgent(null)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-head">
              <h3>{t.shareTitle}: {shareAgent.name}</h3>
              <button type="button" className="btn btn-ghost" onClick={() => setShareAgent(null)}>X</button>
            </div>
            <form className="form-stack" onSubmit={handleShareAgent}>
              <p style={{ fontSize: '0.88rem', color: 'var(--highlight-silver)' }}>{t.shareDesc}</p>
              <select className="input" value={shareCountryCode} onChange={(e) => setShareCountryCode(e.target.value)}>
                {COUNTRY_CODES.map((c) => <option key={c.code} value={c.code}>{c.label}</option>)}
              </select>
              <input
                className="input"
                type="tel"
                placeholder="1012345678"
                value={sharePhone}
                onChange={(e) => setSharePhone(e.target.value)}
                required
              />
              <button className="btn btn-primary" type="submit" disabled={shareBusy}>
                {shareBusy ? '...' : t.shareSend}
              </button>
              {shareFeedback && (
                <div className={`status-banner ${shareFeedback.ok ? 'ok' : 'err'}`}>{shareFeedback.text}</div>
              )}
            </form>
          </div>
        </div>
      )}

      {/* WhatsApp OTP Modal */}
      {waModalOpen && (
        <div className="modal-backdrop" onClick={() => setWaModalOpen(false)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-head">
              <h3>{t.waModalTitle}</h3>
              <button type="button" className="btn btn-ghost" onClick={() => setWaModalOpen(false)}>X</button>
            </div>
            {waStep === 1 ? (
              <form className="form-stack" onSubmit={handleRequestOtp}>
                <p style={{ fontSize: '0.88rem', color: 'var(--highlight-silver)' }}>{t.waStep1Desc}</p>
                <select className="input" value={countryCode} onChange={(e) => setCountryCode(e.target.value)}>
                  {COUNTRY_CODES.map((c) => <option key={c.code} value={c.code}>{c.label}</option>)}
                </select>
                <input className="input" type="tel" placeholder="1012345678" value={phoneNumber} onChange={(e) => setPhoneNumber(e.target.value)} required />
                <button className="btn btn-primary" type="submit" disabled={waBusy}>{waBusy ? '...' : t.waSendCode}</button>
              </form>
            ) : (
              <form className="form-stack" onSubmit={handleVerifyOtp}>
                <p style={{ fontSize: '0.88rem', color: 'var(--highlight-silver)' }}>{t.waStep2Desc}</p>
                <input className="input" type="text" maxLength={6} placeholder="000000" value={otpCode} onChange={(e) => setOtpCode(e.target.value)} required />
                <button className="btn btn-primary" type="submit" disabled={waBusy}>{waBusy ? '...' : t.waVerifyBtn}</button>
                <button type="button" className="btn btn-ghost" onClick={() => setWaStep(1)}>Back</button>
              </form>
            )}
            {waFeedback && (
              <div className={`status-banner ${waFeedback.ok ? 'ok' : 'err'}`}>{waFeedback.text}</div>
            )}
          </div>
        </div>
      )}

      {/* Auth Modal */}
      {authModal && (
        <div className="modal-backdrop" onClick={() => setAuthModal(null)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-head">
              <h3>{authModal === 'signup' ? t.signup : t.login}</h3>
              <button type="button" className="btn btn-ghost" onClick={() => setAuthModal(null)}>X</button>
            </div>
            <form className="form-stack" onSubmit={handleAuthSubmit}>
              {authModal === 'signup' && (
                <input className="input" placeholder="Full Name" value={authForm.name} onChange={(e) => setAuthForm({ ...authForm, name: e.target.value })} required />
              )}
              <input className="input" type="text" placeholder="Username or Email" value={authForm.email} onChange={(e) => setAuthForm({ ...authForm, email: e.target.value })} required />
              <input className="input" type="password" placeholder="Password" value={authForm.password} onChange={(e) => setAuthForm({ ...authForm, password: e.target.value })} required />
              <button className="btn btn-primary" type="submit">{authModal === 'signup' ? t.signup : t.login}</button>
            </form>
            <div className="divider"><span>OR</span></div>
            <button type="button" className="btn" style={{ width: '100%', opacity: 0.6, cursor: 'default' }} disabled>
              {t.googleSoon}
            </button>
            {authMsg && <div className="status-banner err">{authMsg}</div>}
          </div>
        </div>
      )}
    </div>
  );
}
